#![feature(core_intrinsics)]

use crate::agrona::concurrent::atomic_buffer::AtomicBuffer;
use crate::agrona::concurrent::ringbuffer::ring_buffer_descriptor::TRAILER_LENGTH;
use crate::agrona::direct_buffer::DirectBuffer;
use std::ptr;

const SHOULD_BOUNDS_CHECK: bool = false;
pub struct UnsafeBuffer {
    // not sure if to use a bytebuffer
    wrap_adjustment: i32,
    byte_array: Box<[u8]>,
    address_offset: i32,
    capacity: i32,
}

// #[inline]
// #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
// unsafe fn atomic_load<T: Copy>(dst: *const T, order: Ordering) -> T {
//     // SAFETY: the caller must uphold the safety contract for `atomic_load`.
//     unsafe {
//         match order {
//             Relaxed => intrinsics::atomic_load_relaxed(dst),
//             Acquire => intrinsics::atomic_load_acquire(dst),
//             SeqCst => intrinsics::atomic_load_seqcst(dst),
//             Release => panic!("there is no such thing as a release load"),
//             AcqRel => panic!("there is no such thing as an acquire-release load"),
//         }
//     }
// }

impl UnsafeBuffer {
    pub fn new(capacity: usize) -> Self {
        let actual_capacity = capacity + TRAILER_LENGTH as usize;
        let mut tmp = vec![0u8; actual_capacity].into_boxed_slice();
        let tmp2 = tmp.as_mut_ptr();
        UnsafeBuffer {
            wrap_adjustment: 0,
            byte_array: tmp,
            address_offset: 0,
            capacity: actual_capacity as i32,
        }
    }

    fn bounds_check0(&self, index: i32, length: i32) -> Result<(), String> {
        let resulting_position = index + length;
        if index < 0 || length < 0 || resulting_position > self.capacity {
            let tmp = &self.capacity;
            return Err(format!("index={index} length={length} capacity={tmp}"));
        }
        Ok(())
    }

    fn ensure_capacity(&mut self, index: i32, length: i32) -> Result<(), String> {
        if SHOULD_BOUNDS_CHECK {
            self.bounds_check0(index, length)?;
        }
        Ok(())
    }

    fn bounds_check_wrap(&self, offset: i32, length: i32, capacity: i32) -> Result<(), String> {
        if offset < 0 {
            return Err(format!("invalid offset={}", offset));
        }

        if length < 0 {
            return Err(format!("invalid length={}", length));
        }

        if (offset > capacity - length) || (length > capacity - offset) {
            return Err(format!("offset={} length={} not valid for capacity={}", offset, length, capacity));
        }
        Ok(())
    }
}

impl AtomicBuffer for UnsafeBuffer {
    fn verify_alignment(&self) {
        // only need check alignemnt if we are dealing with raw bytes
    }

    fn get_long_volatile(&self, index: i32) -> i64 {
        unsafe {
            let ptr = self.byte_array.as_ptr().add((self.address_offset + index) as usize) as *const i64;
            // atomic_load(ptr, SeqCst);
            ptr::read_volatile(ptr)
        }
    }

    fn put_long_volatile(&self, index: i32, value: i64) {
        todo!()
    }

    fn put_long_ordered(&mut self, index: i32, value: i64) {
        unsafe {
            let ptr = self.byte_array.as_mut_ptr().add((self.address_offset + index) as usize) as *mut i64;
            ptr::write_unaligned(ptr, value);
        }
    }

    fn add_long_ordered(&self, index: i32, increment: i64) {
        todo!()
    }

    fn compare_and_set_long(&self, index: i32, expected_value: i64, update_value: i64) {
        todo!()
    }

    fn get_and_set_long(&self, index: i32, value: i64) -> i64 {
        todo!()
    }

    fn get_and_add_long(&self, index: i32, delta: i64) -> i64 {
        todo!()
    }

    fn get_int_volatile(&self, index: i32) -> i32 {
        unsafe {
            let ptr = self.byte_array.as_ptr().add((self.address_offset + index) as usize) as *const i32;
            ptr::read_volatile(ptr)
        }
    }

    fn put_int_volatile(&mut self, index: i32, value: i32) {
        unsafe {
            let ptr = self.byte_array.as_mut_ptr().add((self.address_offset + index) as usize) as *mut i32;
            ptr::write_volatile(ptr, value);
        }
    }

    fn put_int_ordered(&mut self, index: i32, value: i32) {
        unsafe {
            let ptr = self.byte_array.as_mut_ptr().add((self.address_offset + index) as usize) as *mut i32;
            ptr::write_unaligned(ptr, value);
        }
    }

    fn add_int_ordered(&mut self, index: i32, increment: i32) {
        todo!()
    }

    fn compare_and_set_int(&mut self, index: i32, expected_value: i32, update_value: i32) -> Result<bool, String> {
        todo!()
    }

    fn get_and_set_int(&self, index: i32, value: i32) -> i32 {
        todo!()
    }

    fn get_and_add_int(&self, index: i32, delta: i32) -> i32 {
        todo!()
    }

    fn get_short_volatile(&self, index: i32) -> i16 {
        todo!()
    }

    fn put_short_volatile(&self, index: i32, value: i16) {
        todo!()
    }

    fn get_char_volatile(&self, index: i32) -> char {
        todo!()
    }

    fn put_char_volatile(&self, index: i32, value: char) {
        todo!()
    }

    fn get_byte_volatile(&self, index: i32) -> u8 {
        todo!()
    }

    fn put_byte_ordered(&self, index: i32, value: u8) {
        todo!()
    }
}

impl DirectBuffer for UnsafeBuffer {
    fn address_offset(&self) -> i32 {
        self.address_offset
    }

    fn byte_array(&self) -> &[u8] {
        &self.byte_array
    }

    fn capacity(&self) -> i32 {
        self.capacity
    }

    fn check_limit(&self, limit: i32) -> Result<(), String> {
        if limit > self.capacity {
            return Err(format!("limit={} is beyond capacity={}", limit, self.capacity));
        }
        Ok(())
    }

    fn get_long(&self, index: i32) -> i64 {
        unsafe {
            let ptr = self.byte_array.as_ptr().add((self.address_offset + index) as usize) as *const i64;
            ptr::read_unaligned(ptr)
        }
    }

    fn get_int(&self, index: i32) -> i32 {
        unsafe {
            let ptr = self.byte_array.as_ptr().add((self.address_offset + index) as usize) as *const i32;
            ptr::read_unaligned(ptr)
        }
    }

    fn parse_natural_int_ascii(&self, index: i32, length: i32) -> i32 {
        todo!()
    }

    fn parse_natural_long_ascii(&self, index: i32, length: i32) -> i64 {
        todo!()
    }

    fn parse_int_ascii(&self, index: i32, length: i32) -> i32 {
        todo!()
    }

    fn parse_long_ascii(&self, index: i32, length: i32) -> i64 {
        todo!()
    }

    fn get_double(&self, index: i32) -> f64 {
        todo!()
    }

    fn get_float(&self, index: i32) -> f32 {
        todo!()
    }

    fn get_short(&self, index: i32) -> i16 {
        todo!()
    }

    fn get_char(&self, index: i32) -> char {
        todo!()
    }

    // fn get_string_ascii(&self, index: i32) -> String {
    //     todo!()
    // }

    // fn get_string_without_length_ascii(&self, index: i32, length: i32) -> String {
    //     todo!()
    // }

    // fn get_string_utf8(&self, index: i32) -> String {
    //     todo!()
    // }

    // fn get_string_utf8_0(&self, index: i32, length: i32) -> String {
    //     todo!()
    // }

    // fn get_string_without_length_utf8(&self, index: i32, length: i32) -> String {
    //     todo!()
    // }

    // fn get_byte(&self, index: i32) -> u8 {
    //     todo!()
    // }
    //
    // fn get_bytes(&self, index: i32, dst: &[u8]) {
    //     todo!()
    // }
    //
    // fn get_bytes0(&self, index: i32, dst: &[u8], offset: i32, length: i32) {
    //     todo!()
    // }
    //
    // fn get_bytes1(&self, index: i32, dst_buffer: &UnsafeBuffer, offset: i32, length: i32) {
    //     todo!()
    // }
    //
    // fn bounds_check(&self, index: i32, length: i32) {
    //     todo!()
    // }

    fn wrap_adjustment(&self) -> i32 {
        self.wrap_adjustment
    }

    // pub unsafe fn put_char_volatile(o: *mut u8, offset: i32, x: char) {
    //     let x_as_u16 = x as u16;
    //     let addr = o.add(offset) as *mut u16;
    //     ptr::write_volatile(addr, x_as_u16);
    //     compiler_fence(Ordering::SeqCst);
    // }

    fn is_expandable(&self) -> bool {
        false
    }

    fn set_memory(&self, index: i32, length: i32, value: u8) {
        todo!()
    }

    fn put_long(&mut self, index: i32, value: i64) {
        unsafe {
            let ptr = self.byte_array.as_mut_ptr().add((self.address_offset + index) as usize) as *mut i64;
            ptr::write_unaligned(ptr, value);
        }
    }

    fn put_int(&mut self, index: i32, value: i32) {
        unsafe {
            let ptr = self.byte_array.as_mut_ptr().add((self.address_offset + index) as usize) as *mut i32;
            ptr::write_unaligned(ptr, value);
        }
    }

    fn put_int_ascii(&self, index: i32, value: i32) -> i32 {
        todo!()
    }

    fn put_natural_int_ascii(&self, index: i32, value: i32) -> i32 {
        todo!()
    }

    fn put_natural_padding_int_ascii(&self, index: i32, length: i32, value: i64) {
        todo!()
    }

    fn put_natural_int_ascii_from_end(&self, value: i32, end_exclusive: i32) -> i32 {
        todo!()
    }

    fn put_natural_long_ascii(&self, index: i32, value: i64) -> i32 {
        todo!()
    }

    fn put_long_ascii(&self, index: i32, value: i64) -> i32 {
        todo!()
    }

    fn put_double(&self, index: i32, value: f64) {
        todo!()
    }

    fn put_float(&self, index: i32, value: f32) {
        todo!()
    }

    fn put_short(&self, index: i32, value: i16) {
        todo!()
    }

    fn put_char(&self, index: i32, value: char) {
        todo!()
    }

    fn put_byte(&self, index: i32, value: u8) {
        todo!()
    }

    fn put_bytes(&self, index: i32, bytes: &UnsafeBuffer) {
        todo!()
    }

    fn put_bytes2(&self, index: i32, src_buffer: &UnsafeBuffer, offset: i32, length: i32) {
        todo!()
    }

    fn put_string_ascii(&self, index: i32, value: &str) -> i32 {
        todo!()
    }

    fn put_string_without_length_ascii(&self, index: i32, value: &str) -> i32 {
        todo!()
    }

    fn put_string_without_length_ascii0(&self, index: i32, value: &str, value_offset: i32, length: i32) -> i32 {
        todo!()
    }

    fn put_string_utf8(&self, index: i32, value: &str) -> i32 {
        todo!()
    }

    fn put_string_utf8_1(&self, index: i32, value: &str, max_encoded_length: i32) -> i32 {
        todo!()
    }

    fn put_string_without_length_utf8(&self, index: i32, value: &str) -> i32 {
        todo!()
    }
}
