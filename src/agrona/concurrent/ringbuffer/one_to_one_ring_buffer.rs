use std::cmp::max;
use crate::agrona::concurrent::atomic_buffer::AtomicBuffer;
use crate::agrona::concurrent::controlled_message_handler::{Action, ControlledMessageHandler};
use crate::agrona::concurrent::message_handler::MessageHandler;
use crate::bit_util::{align};
use crate::agrona::concurrent::ringbuffer::ring_buffer::{RingBuffer, INSUFFICIENT_CAPACITY, PADDING_MSG_TYPE_ID};
use crate::agrona::concurrent::ringbuffer::record_descriptor::{check_type_id, encoded_msg_offset, length_offset, type_offset, ALIGNMENT, HEADER_LENGTH};
use crate::agrona::concurrent::ringbuffer::ring_buffer_descriptor::{check_capacity, CONSUMER_HEARTBEAT_OFFSET, CORRELATION_COUNTER_OFFSET, HEAD_CACHE_POSITION_OFFSET, HEAD_POSITION_OFFSET, TAIL_POSITION_OFFSET};
use crate::agrona::direct_buffer::DirectBuffer;

const MIN_CAPACITY: usize = HEADER_LENGTH << 1;

pub struct OneToOneRingBuffer {
    capacity: usize,
    max_msg_length: usize,
    tail_position_index: usize,
    head_cache_position_index: usize,
    head_position_index: usize,
    correlation_id_counter_index: usize,
    consumer_heartbeat_index: usize,
    buffer: Box<dyn AtomicBuffer>,
}

impl OneToOneRingBuffer {
    fn new(buffer: Box<dyn AtomicBuffer>) -> Self {

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

    fn check_msg_length(length: usize) {
        if length < 0 {
            panic!("Invalid message length={}", length);
        } else if length > Self.max_msg_length {
            panic!("Encoded message exceeds max_msg_length={}", length);
        }
    }

    fn claim_capacity(&self, buffer: Box<dyn AtomicBuffer>, record_length: usize) -> usize {
        let aligned_record_length = align(record_length, HEADER_LENGTH);
        let required_capacity = aligned_record_length + HEADER_LENGTH;
        let mask = Self.capacity - 1;

        let head = buffer.getLong(Self.head_cache_position_index);
        let tail = buffer.getLong(Self.tail_position_index);
        let available_capacity = Self.capacity - (tail - head);
        if required_capacity > available_capacity {
            head = buffer.getLongVolatile(Self.head_position_index);
            if required_capacity > Self.capacity - (tail - head) {
                INSUFFICIENT_CAPACITY
            }
            buffer.putLong(Self.head_cache_position_index, head);
        }

        let mut padding = 0;
        let record_index = tail & mask;
        let to_buffer_end_length = Self.capacity - record_index;
        let write_index = record_index;
        let next_tail = tail + aligned_record_length;

        if aligned_record_length == to_buffer_end_length { // message fits within the end of the buffer
            buffer.putLongOrdered(Self.tail_position_index, next_tail);
            buffer.putLong(0, 0); // pre-zero next message header
            record_index
        } else if required_capacity > to_buffer_end_length {
            write_index = 0;
            let head_index = head & mask;

            if required_capacity > head_index {
                head = buffer.get_long_volatile(Self.head_position_index);
                head_index = head & mask;
                if required_capacity > head_index {
                    write_index = INSUFFICIENT_CAPACITY;
                    next_tail = tail;
                }

                buffer.put_long(Self.head_cache_position_index, head);
            }

            padding = to_buffer_end_length;
            next_tail += padding;
        }

        buffer.put_long_ordered(Self.tail_position_index, next_tail);

        if padding != 0 {
            buffer.put_long(0, 0);
            buffer.put_int_ordered(length_offset(record_index), -padding);
            // VarHandle.releaseFence()

            buffer.put_int(type_offset(record_index), PADDING_MSG_TYPE_ID);
            buffer.put_int_ordered(length_offset(record_index), padding);
        }

        if write_index != INSUFFICIENT_CAPACITY {
            buffer.put_long(write_index + aligned_record_length, 0); // pre-zero next message header
        }

        record_index;
    }

    fn compute_record_index(index: usize) -> usize {
        let record_index = index - HEADER_LENGTH;
        if record_index < 0 || record_index > (Self.capacity - HEADER_LENGTH) {
            panic!("Invalid message index={}", index);
        }
        record_index
    }

    fn verify_claimed_space_not_released(buffer: Box<dyn AtomicBuffer>, record_index: usize) -> usize {
        let record_length = buffer.get_int(length_offset(record_index));
        if record_length < 0 {
            record_index
        }
        if PADDING_MSG_TYPE_ID == buffer.get_int(type_offset(record_index)) {
            panic!("claimed space previously aborted");
        } else {
            panic!("claimed space previously committed");
        }
    }
}

impl RingBuffer for OneToOneRingBuffer {
    fn capacity(&self) -> usize {
        self.capacity
    }

    fn write(&self, msg_type_id: i32, src_buffer: Box<dyn DirectBuffer>, offset: usize, length: usize) -> bool {
        check_type_id(msg_type_id);
        Self::check_msg_length(length);

        let record_length = length + HEADER_LENGTH;
        let record_index = Self::claim_capacity(Self.buffer, record_length);

        if record_index == INSUFFICIENT_CAPACITY as usize {
            return false
        }

        Self.buffer.put_int_ordered(length_offset(record_index), -1 * record_length);
        // VarHandle.releaseFence()

        Self.buffer.put_bytes(encoded_msg_offset(record_index), src_buffer, offset, length);
        Self.buffer.put_int(type_offset(record_index), msg_type_id);
        Self.buffer.put_int_ordered(length_offset(record_index), 0);

        true
    }

    fn try_claim(&self, msg_type_id: i32, length: usize) -> usize {
        check_type_id(msg_type_id);
        Self::check_msg_length(length);

        let record_length = length + HEADER_LENGTH;
        let record_index = Self::claim_capacity(Self.buffer, record_length);

        if record_index == INSUFFICIENT_CAPACITY as usize {
            return record_index
        }

        Self.buffer.put_int_ordered(length_offset(record_index), -1 * record_length);
        // VarHandle.releaseFence();
        Self.buffer.put_int(type_offset(record_index), msg_type_id);

        encoded_msg_offset(record_index);
    }

    fn commit(&self, index: usize) {
        let record_index = Self::compute_record_index(index);
        let record_length = Self::verify_claimed_space_not_released(Self.buffer, record_index);

        Self.buffer.put_int_ordered(length_offset(record_index), -1 * record_length);
    }

    fn abort(&self, index: usize) {
        let record_index = Self::compute_record_index(index);
        let record_length = Self::verify_claimed_space_not_released(Self.buffer, record_index);

        Self.buffer.put_int(type_offset(record_index), PADDING_MSG_TYPE_ID);
        Self.buffer.put_int_ordered(length_offset(record_index), -1 * record_length);
    }

    // this means dynamically dispatched trait object
    fn read(&self, handler: &dyn MessageHandler) -> i32 {
        Self::read0(self, handler, i32::MAX)
    }

    fn read0(&self, handler: &dyn MessageHandler, message_count_limit: i32) -> i32 {
        let mut messages_read = 0;

        let head_position_index = Self.head_position_index;
        let head = Self.buffer.get_long(head_position_index);

        let mut bytes_read = 0;

        let capacity = Self.capacity;
        let head_index = head & (capacity - 1);
        let contiguous_block_length = capacity - head_index;

        while (bytes_read < contiguous_block_length) && (messages_read < message_count_limit) {
            let record_index = head_index + bytes_read;
            let record_length = Self.buffer.get_int_volatile(length_offset(record_index));
            if record_length < 0 {
                break;
            }

            bytes_read += align(record_length, ALIGNMENT);

            let message_type_id = Self.buffer.get_int(type_offset(record_index));
            if message_type_id == PADDING_MSG_TYPE_ID {
                continue;
            }

            handler.on_message(message_type_id, Self.buffer, record_index + HEADER_LENGTH, record_length - HEADER_LENGTH);
            messages_read += 1;
        }
    }

    fn controlled_read(&self, handler: &dyn ControlledMessageHandler) {
        Self::controlled_read0(self, handler, i32::MAX);
    }

    fn controlled_read0(&self, handler: &dyn ControlledMessageHandler, message_count_limit: i32) {
        let mut messages_read = 0;

        let head_position_index = Self.head_position_index;
        let head = Self.buffer.get_long(head_position_index);

        let mut bytes_read = 0;

        let capacity = Self.capacity;
        let head_index = head & (capacity - 1);
        let contiguous_block_length = capacity - head_index;

        // try/finally need refactor
        while (bytes_read < contiguous_block_length) && (messages_read < message_count_limit) {
            let record_index = head_index + bytes_read;
            let record_length = Self.buffer.get_int_volatile(length_offset(record_index));
            if record_length < 0 {
                break;
            }

            let aligned_length = align(record_length, ALIGNMENT);
            bytes_read += aligned_length;

            let message_type_id = Self.buffer.get_int(type_offset(record_index));
            if message_type_id == PADDING_MSG_TYPE_ID {
                continue;
            }

            let action = handler.on_message(message_type_id, Self.buffer, record_index + HEADER_LENGTH, record_length - HEADER_LENGTH);
            if action == Action::ABORT {
                bytes_read -= aligned_length;
                break;
            }

            messages_read += 1;

            if action == Action::BREAK {
                break;
            }

            if action == Action::COMMIT {
                Self.buffer.put_long_ordered(head_position_index, head + bytes_read);
                head_index += bytes_read;
                head += bytes_read;
                bytes_read = 0;
            }
        }
        if bytes_read > 0 {
            Self.buffer.put_long_ordered(head_position_index, head + bytes_read);
        }

        messages_read
    }

    fn max_msg_length(&self) -> usize {
        Self.max_msg_length
    }

    fn next_correlation_id(&self) -> i64 {
        Self.buffer.get_and_add_long(Self.correlation_id_counter_index, 1);
    }

    fn buffer(&self) -> Box<dyn AtomicBuffer> {
        Self.buffer
    }

    fn put_consumer_heartbeat_time(&self, time: i64) {
        Self.buffer.put_long_ordered(Self.consumer_heartbeat_index, time);
    }

    fn consumer_heartbeat_time(&self) -> i64 {
        Self.buffer.get_long_volatile(Self.consumer_heartbeat_index)
    }

    fn producer_position(&self) -> i64 {
        Self.buffer.get_long_volatile(Self.tail_position_index);
    }

    fn consumer_position(&self) -> i64 {
        Self.buffer.get_long_volatile(Self.head_position_index);
    }

    fn size(&self) -> usize {
        let mut head_before;
        let mut tail;
        let mut head_after = Self.buffer.get_long_volatile(Self.head_position_index);

        loop {
            head_before = head_after;
            tail = Self.buffer.get_long_volatile(Self.tail_position_index);
            head_after = Self.buffer.get_long_volatile(Self.head_position_index);

            if head_before == head_before {
                break;
            }
        }

        let size = tail - head_after;
        if size < 0 {
            0
        } else if size > Self.capacity {
            Self.capacity
        }

        size
    }

    fn unblock(&self) -> bool {
        false
    }
}
