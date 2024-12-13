use std::cell::UnsafeCell;
use crate::agrona::concurrent::atomic_buffer::AtomicBuffer;
use crate::agrona::concurrent::controlled_message_handler::Action;
use crate::agrona::concurrent::ringbuffer::record_descriptor::{check_type_id, encoded_msg_offset, length_offset, type_offset, ALIGNMENT, HEADER_LENGTH};
use crate::agrona::concurrent::ringbuffer::ring_buffer::{RingBuffer, INSUFFICIENT_CAPACITY, PADDING_MSG_TYPE_ID};
use crate::agrona::concurrent::ringbuffer::ring_buffer_descriptor::{check_capacity, CONSUMER_HEARTBEAT_OFFSET, CORRELATION_COUNTER_OFFSET, HEAD_CACHE_POSITION_OFFSET, HEAD_POSITION_OFFSET, TAIL_POSITION_OFFSET};
use crate::agrona::concurrent::unsafe_buffer::UnsafeBuffer;
use crate::agrona::direct_buffer::DirectBuffer;
use crate::bit_util::align;
use std::cmp::max;
use std::sync::atomic::{fence, Ordering};

const MIN_CAPACITY: i32 = HEADER_LENGTH << 1;

pub struct OneToOneRingBuffer {
    capacity: i32,
    max_msg_length: i32,
    tail_position_index: i32,
    head_cache_position_index: i32,
    head_position_index: i32,
    correlation_id_counter_index: i32,
    consumer_heartbeat_index: i32,
    buffer: UnsafeCell<UnsafeBuffer>
}

unsafe impl Send for OneToOneRingBuffer {}

unsafe impl Sync for OneToOneRingBuffer {}

impl OneToOneRingBuffer {
    pub fn new(buffer: UnsafeBuffer) -> Self {
        let capacity = check_capacity(buffer.capacity(), MIN_CAPACITY);
        buffer.verify_alignment();
        let max_msg_length = if MIN_CAPACITY == capacity {
            0
        } else {
            max(HEADER_LENGTH, capacity >> 3)
        };
        let tail_position_index = capacity + TAIL_POSITION_OFFSET;
        let head_cache_position_index = capacity + HEAD_CACHE_POSITION_OFFSET;
        let head_position_index = capacity + HEAD_POSITION_OFFSET;
        let correlation_id_counter_index = capacity + CORRELATION_COUNTER_OFFSET;
        let consumer_heartbeat_index = capacity + CONSUMER_HEARTBEAT_OFFSET;
        let buffer = UnsafeCell::new(buffer);

        OneToOneRingBuffer {
            capacity,
            max_msg_length,
            tail_position_index,
            head_cache_position_index,
            head_position_index,
            correlation_id_counter_index,
            consumer_heartbeat_index,
            buffer
        }
    }

    fn check_msg_length(&self, length: i32) {
        if length < 0 {
            panic!("Invalid message length={}", length);
        } else if length > self.max_msg_length {
            panic!("Encoded message exceeds max_msg_length={}", length);
        }
    }

    fn claim_capacity(&self, record_length: i32) -> i32 {
        let aligned_record_length = align(record_length, ALIGNMENT);
        let required_capacity = aligned_record_length + HEADER_LENGTH;
        let mask = self.capacity - 1;
        let buffer = unsafe {
            &mut *self.buffer.get()
        };
        let mut head = buffer.get_long(self.head_cache_position_index);
        let tail = buffer.get_long(self.tail_position_index);
        let available_capacity = self.capacity - (tail - head) as i32;

        if required_capacity > available_capacity {
            head = buffer.get_long_volatile(self.head_position_index);
            if required_capacity > self.capacity - (tail - head) as i32 {
                return INSUFFICIENT_CAPACITY;
            }
            buffer.put_long(self.head_cache_position_index, head);
        }

        let mut padding = 0;
        let record_index = tail as i32 & mask;
        let to_buffer_end_length = self.capacity - record_index;
        let mut write_index = record_index;
        let mut next_tail = tail + aligned_record_length as i64;
        // println!("tail, arl, next_tail: {} {} {}", tail, aligned_record_length, next_tail);

        if aligned_record_length == to_buffer_end_length { // message fits within the end of the buffer
            // println!("here6: {}", next_tail);
            buffer.put_long_ordered(self.tail_position_index, next_tail);
            buffer.put_long(0, 0); // pre-zero next message header
            return record_index;
        } else if required_capacity > to_buffer_end_length {
            write_index = 0;
            let mut head_index = head as i32 & mask;

            if required_capacity > head_index {
                head = buffer.get_long_volatile(self.head_position_index);
                head_index = head as i32 & mask;
                if required_capacity > head_index {
                    write_index = INSUFFICIENT_CAPACITY;
                    next_tail = tail;
                }

                buffer.put_long(self.head_cache_position_index, head);
            }

            padding = to_buffer_end_length;
            next_tail += padding as i64;
        }

        buffer.put_long_ordered(self.tail_position_index, next_tail);
        // println!("here2: {}, {}", next_tail, buffer.get_long(self.tail_position_index));

        if padding != 0 {
            buffer.put_long(0, 0);
            buffer.put_int_ordered(length_offset(record_index), -padding);
            // fence(Ordering::Release);

            buffer.put_int(type_offset(record_index), PADDING_MSG_TYPE_ID);
            buffer.put_int_ordered(length_offset(record_index), padding);
        }

        if write_index != INSUFFICIENT_CAPACITY {
            buffer.put_long(write_index + aligned_record_length, 0); // pre-zero next message header
            // println!("here3: {}, {}", 0, buffer.get_long(write_index + aligned_record_length));
        }

        write_index
    }

    fn compute_record_index(&self, index: i32) -> i32 {
        let record_index = index - HEADER_LENGTH;
        if record_index < 0 || record_index > (self.capacity - HEADER_LENGTH) {
            panic!("Invalid message index={}", index);
        }
        record_index
    }

    fn verify_claimed_space_not_released(&self, buffer: &UnsafeBuffer, record_index: i32) -> i32 {
        let record_length = buffer.get_int(length_offset(record_index));
        if record_length < 0 {
            return record_length;
        }
        if PADDING_MSG_TYPE_ID == buffer.get_int(type_offset(record_index)) {
            panic!("claimed space previously aborted");
        } else {
            panic!("claimed space previously committed");
        }
    }
}

impl RingBuffer for OneToOneRingBuffer {
    fn capacity(&self) -> i32 {
        self.capacity
    }

    fn write(&self, msg_type_id: i32, src_buffer: &UnsafeBuffer, offset: i32, length: i32) -> bool {
        check_type_id(msg_type_id);
        let buffer = unsafe {
            &mut *self.buffer.get()
        };
        Self::check_msg_length(self, length);

        let record_length = length + HEADER_LENGTH;
        let record_index = Self::claim_capacity(self, record_length);

        if record_index == INSUFFICIENT_CAPACITY {
            return false
        }

        buffer.put_int_ordered(length_offset(record_index), -1 * record_length);
        // fence(Ordering::Release);

        buffer.put_bytes2(encoded_msg_offset(record_index), src_buffer, offset, length);
        buffer.put_int(type_offset(record_index), msg_type_id);
        buffer.put_int_ordered(length_offset(record_index), record_length);

        true
    }

    fn try_claim(&self, msg_type_id: i32, length: i32) -> i32 {
        let buffer = unsafe {
            &mut *self.buffer.get()
        };
        check_type_id(msg_type_id);
        Self::check_msg_length(self, length);

        let record_length = length + HEADER_LENGTH;
        let record_index = Self::claim_capacity(self, record_length);

        if record_index == INSUFFICIENT_CAPACITY {
            return record_index
        }


        buffer.put_int_ordered(length_offset(record_index), -1 * record_length);
        // println!("here: {}, {}", -1 * record_length, buffer.get_int(length_offset(record_index)));
        // fence(Ordering::Release);
        buffer.put_int(type_offset(record_index), msg_type_id);
        // println!("here1: {}, {}", msg_type_id, buffer.get_int(type_offset(record_index)));

        encoded_msg_offset(record_index)
    }

    fn commit(&self, index: i32) {
        let buffer = unsafe {
            &mut *self.buffer.get()
        };
        let record_index = Self::compute_record_index(&self, index);
        let record_length = Self::verify_claimed_space_not_released(&self, &buffer, record_index);
        buffer.put_int_ordered(length_offset(record_index), -1 * record_length);
        // println!("here4: {}, {}", -1 * record_length, buffer.get_int(length_offset(record_index)));
    }

    fn abort(&self, index: i32) {
        let buffer = unsafe {
            &mut *self.buffer.get()
        };
        let record_index = Self::compute_record_index(&self, index);
        let record_length = Self::verify_claimed_space_not_released(&self, &buffer, record_index);

        buffer.put_int(type_offset(record_index), PADDING_MSG_TYPE_ID);
        buffer.put_int_ordered(length_offset(record_index), -1 * record_length);
    }

    // this means dynamically dispatched trait object
    fn read<F>(&self, func: F) -> i32 where F: FnMut(i32, &UnsafeBuffer, i32, i32) {
        Self::read0(self, func, i32::MAX)
    }

    fn read0<F>(&self, mut func: F, message_count_limit: i32) -> i32 where F: FnMut(i32, &UnsafeBuffer, i32, i32) {
        let buffer = unsafe {
            &mut *self.buffer.get()
        };
        let mut messages_read = 0;

        let head_position_index = self.head_position_index;
        let head = buffer.get_long(head_position_index);

        let mut bytes_read = 0;

        let capacity = self.capacity;
        let head_index = head as i32 & (capacity - 1);
        let contiguous_block_length = capacity - head_index;

        while (bytes_read < contiguous_block_length) && (messages_read < message_count_limit) {
            let record_index = head_index + bytes_read;
            let record_length = buffer.get_int(length_offset(record_index));
            if record_length <= 0 {
                break;
            }

            bytes_read += align(record_length, ALIGNMENT);

            let message_type_id = buffer.get_int(type_offset(record_index));
            if message_type_id == PADDING_MSG_TYPE_ID {
                continue;
            }

            func(message_type_id, &buffer, record_index + HEADER_LENGTH, record_length - HEADER_LENGTH);
            messages_read += 1;
        }
        if bytes_read > 0 {
            buffer.put_long_ordered(head_position_index, head + bytes_read as i64);
        }
        messages_read
    }

    fn controlled_read<F>(&self, func: F) -> i32 where F: Fn(i32, &UnsafeBuffer, i32, i32) -> Action {
        Self::controlled_read0(self, func, i32::MAX)
    }

    fn controlled_read0<F>(&self, func: F, message_count_limit: i32) -> i32 where F: Fn(i32, &UnsafeBuffer, i32, i32) -> Action {
        let mut messages_read = 0;
        let buffer = unsafe {
            &mut *self.buffer.get()
        };

        let head_position_index = self.head_position_index;
        let mut head = buffer.get_long(head_position_index);

        let mut bytes_read = 0;

        let capacity = self.capacity;
        let mut head_index = head as i32 & (capacity - 1);
        let contiguous_block_length = capacity - head_index;

        // try/finally need refactor
        while (bytes_read < contiguous_block_length) && (messages_read < message_count_limit) {
            let record_index = head_index + bytes_read;
            let record_length = buffer.get_int_volatile(length_offset(record_index));
            if record_length < 0 {
                break;
            }

            let aligned_length = align(record_length, ALIGNMENT);
            bytes_read += aligned_length;

            let message_type_id = buffer.get_int(type_offset(record_index));
            if message_type_id == PADDING_MSG_TYPE_ID {
                continue;
            }

            let action = func(message_type_id, &buffer, record_index + HEADER_LENGTH, record_length - HEADER_LENGTH);

            messages_read += 1;

            match action {
                Action::ABORT => {
                    bytes_read -= aligned_length;
                }
                Action::COMMIT => {
                    messages_read += 1;
                    buffer.put_long_ordered(head_position_index, head + bytes_read as i64);
                    head_index += bytes_read;
                    head += bytes_read as i64;
                    bytes_read = 0;
                }
                Action::BREAK => {
                    messages_read += 1;
                    break
                },
                Action::CONTINUE => {}
            }
        }
        if bytes_read > 0 {
            buffer.put_long_ordered(head_position_index, head + bytes_read as i64);
        }

        messages_read
    }

    fn max_msg_length(&self) -> i32 {
        self.max_msg_length
    }

    fn next_correlation_id(&self) -> i64 {
        let buffer = unsafe {
            &mut *self.buffer.get()
        };
        buffer.get_and_add_long(self.correlation_id_counter_index, 1)
    }

    fn buffer(&self) -> &mut UnsafeBuffer {
        let buffer = unsafe {
            &mut *self.buffer.get()
        };
        buffer
    }

    fn put_consumer_heartbeat_time(&self, time: i64) {
        let buffer = unsafe {
            &mut *self.buffer.get()
        };
        buffer.put_long_ordered(self.consumer_heartbeat_index, time);
    }

    fn consumer_heartbeat_time(&self) -> i64 {
        let buffer = unsafe {
            &mut *self.buffer.get()
        };
        buffer.get_long_volatile(self.consumer_heartbeat_index)
    }

    fn producer_position(&self) -> i64 {
        let buffer = unsafe {
            &mut *self.buffer.get()
        };
        buffer.get_long_volatile(self.tail_position_index)
    }

    fn consumer_position(&self) -> i64 {
        let buffer = unsafe {
            &mut *self.buffer.get()
        };
        buffer.get_long_volatile(self.head_position_index)
    }

    fn size(&self) -> i32 {
        let buffer = unsafe {
            &mut *self.buffer.get()
        };
        let mut head_before;
        let mut tail;
        let mut head_after = buffer.get_long_volatile(self.head_position_index);

        loop {
            head_before = head_after;
            tail = buffer.get_long_volatile(self.tail_position_index);
            head_after = buffer.get_long_volatile(self.head_position_index);

            if head_before == head_before {
                break;
            }
        }

        let size = tail - head_after;
        if size < 0 {
            return 0;
        } else if size > self.capacity as i64 {
            return self.capacity;
        }

        size as i32
    }

    fn unblock(&self) -> bool {
        false
    }
}
