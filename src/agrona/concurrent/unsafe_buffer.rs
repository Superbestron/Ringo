use std::ptr;
use std::sync::atomic::{compiler_fence, Ordering};
use byteorder::ByteOrder;
use crate::agrona::concurrent::atomic_buffer;
use crate::agrona::concurrent::atomic_buffer::AtomicBuffer;
use crate::agrona::direct_buffer::DirectBuffer;
use crate::agrona::mutable_direct_buffer::MutableDirectBuffer;

const SHOULD_BOUNDS_CHECK: bool = false;
pub struct UnsafeBuffer {
    // not sure if to use a bytebuffer
    wrap_adjustment: usize,
    byte_array: [u8],
    address_offset: usize,
    capacity: usize,
}

impl UnsafeBuffer {
    fn new() -> Self {
        // UnsafeBuffer {
        //
        // }
    }

    fn bounds_check_wrap(&self, offset: usize, length: usize, capacity: usize) -> Result<(), String> {
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
        // self.byte_array != null
    }

    fn get_long_volatile(&self, index: usize) -> i64 {
        todo!()
    }

    fn put_long_volatile(&self, index: usize, value: i64) {
        todo!()
    }

    fn put_long_ordered(&self, index: usize, value: i64) {
        todo!()
    }

    fn add_long_ordered(&self, index: usize, increment: i64) {
        todo!()
    }

    fn compare_and_set_long(&self, index: usize, expected_value: i64, update_value: i64) {
        todo!()
    }

    fn get_and_set_long(&self, index: usize, value: i64) -> i64 {
        todo!()
    }

    fn get_and_add_long(&self, index: usize, delta: i64) -> i64 {
        todo!()
    }

    fn get_int_volatile(&self, index: usize) -> i32 {
        todo!()
    }

    fn put_int_volatile(&self, index: usize, value: i32) {
        todo!()
    }

    fn put_int_ordered(&self, index: usize, value: i32) {
        todo!()
    }

    fn add_int_ordered(&self, index: usize, increment: i32) {
        todo!()
    }

    fn compare_and_set_int(&self, index: usize, expected_value: i32, update_value: i32) {
        todo!()
    }

    fn get_and_set_int(&self, index: usize, value: i32) -> i32 {
        todo!()
    }

    fn get_and_add_int(&self, index: usize, delta: i32) -> i32 {
        todo!()
    }

    fn get_short_volatile(&self, index: usize) -> i16 {
        todo!()
    }

    fn put_short_volatile(&self, index: usize, value: i16) {
        todo!()
    }

    fn get_char_volatile(&self, index: usize) -> char {
        todo!()
    }

    fn put_char_volatile(&self, index: usize, value: char) {
        todo!()
    }

    fn get_byte_volatile(&self, index: usize) -> u8 {
        todo!()
    }

    fn put_byte_ordered(&self, index: usize, value: u8) {
        todo!()
    }
}

impl MutableDirectBuffer for UnsafeBuffer {
    fn is_expandable(&self) -> bool {
        false
    }

    fn set_memory(&self, index: usize, length: usize, value: u8) {
        todo!()
    }

    fn put_long_0(&self, index: usize, value: i64, byte_order: &dyn ByteOrder) {
        todo!()
    }

    fn put_long(&self, index: usize, value: i64) {
        todo!()
    }

    fn put_int_0(&self, index: usize, value: i64, byte_order: &dyn ByteOrder) {
        todo!()
    }

    fn put_int(&self, index: usize, value: i64) {
        todo!()
    }

    fn put_int_ascii(&self, index: usize, value: i32) -> usize {
        todo!()
    }

    fn put_natural_int_ascii(&self, index: usize, value: i32) -> usize {
        todo!()
    }

    fn put_natural_padding_int_ascii(&self, index: usize, length: usize, value: i64) {
        todo!()
    }

    fn put_natural_int_ascii_from_end(&self, value: i32, end_exclusive: i32) -> usize {
        todo!()
    }

    fn put_natural_long_ascii(&self, index: usize, value: i64) -> usize {
        todo!()
    }

    fn put_long_ascii(&self, index: usize, value: i64) -> usize {
        todo!()
    }

    fn put_double0(&self, index: usize, value: f64, byte_order: &dyn ByteOrder) {
        todo!()
    }

    fn put_double(&self, index: usize, value: f64) {
        todo!()
    }

    fn put_float0(&self, index: usize, value: f32, byte_order: &dyn ByteOrder) {
        todo!()
    }

    fn put_float(&self, index: usize, value: f32) {
        todo!()
    }

    fn put_short0(&self, index: usize, value: i16, byte_order: &dyn ByteOrder) {
        todo!()
    }

    fn put_short(&self, index: usize, value: i16) {
        todo!()
    }

    fn put_char0(&self, index: usize, value: char, byte_order: &dyn ByteOrder) {
        todo!()
    }

    fn put_char(&self, index: usize, value: char) {
        todo!()
    }

    fn put_byte(&self, index: usize, value: u8) {
        todo!()
    }

    fn put_bytes(&self, index: usize, bytes: &[u8]) {
        todo!()
    }

    fn put_bytes0(&self, index: usize, byte_order: &dyn ByteOrder, offset: usize, length: usize) {
        todo!()
    }

    fn put_bytes2(&self, index: usize, src_buffer: Box<dyn DirectBuffer>, offset: usize, bytes: &[u8]) {
        todo!()
    }

    fn put_string_ascii(&self, index: usize, value: &str) -> usize {
        todo!()
    }

    fn put_string_ascii0(&self, index: usize, value: &str, byte_order: &dyn ByteOrder) -> usize {
        todo!()
    }

    fn put_string_without_length_ascii(&self, index: usize, value: &str) -> usize {
        todo!()
    }

    fn put_string_without_length_ascii0(&self, index: usize, value: &str, value_offset: usize, length: usize) -> usize {
        todo!()
    }

    fn put_string_utf8(&self, index: usize, value: &str) -> usize {
        todo!()
    }

    fn put_string_utf8_0(&self, index: usize, value: &str, byte_order: &dyn ByteOrder) -> usize {
        todo!()
    }

    fn put_string_utf8_1(&self, index: usize, value: &str, max_encoded_length: usize) -> usize {
        todo!()
    }

    fn put_string_utf8_2(&self, index: usize, value: &str, byte_order: &dyn ByteOrder, max_encoded_length: usize) -> usize {
        todo!()
    }

    fn put_string_without_length_utf8(&self, index: usize, value: &str) -> usize {
        todo!()
    }
}

impl DirectBuffer for UnsafeBuffer {
    fn wrap(&self, buffer: &[u8]) -> Result<(), String> {
        Self.capacity = buffer.len();
        Self.address_offset = 0; // arr_base_offset
        Self.wrap_adjustment = 0;

        unsafe {
            if Self.byte_array.as_ptr() != buffer.as_ptr() {
                Self.byte_array = *std::slice::from_raw_parts(buffer.as_ptr(), buffer.len());
            }
        }
        Ok(())
    }

    fn wrap0(&mut self, buffer: &[u8], offset: usize, length: usize) -> Result<(), String> {
        if SHOULD_BOUNDS_CHECK {
            self.bounds_check_wrap(offset, length, buffer.len())?;
        }
        self.bounds_check_wrap(offset, length, buffer.len()).unwrap();
        self.capacity = length;
        self.address_offset = 0 + offset;
        self.wrap_adjustment = offset;

        unsafe {
            if self.byte_array.as_ptr() != buffer.as_ptr() {
                self.byte_array = *std::slice::from_raw_parts(buffer.as_ptr(), buffer.len());
                // self.byte_array = buffer.to_vec()
            }
        }
        Ok(())
    }

    fn wrap1(&self, buffer: Box<dyn DirectBuffer>) {
        Self.capacity = buffer.len();
        Self.address_offset = 0; // arr_base_offset
        Self.wrap_adjustment = 0;

        unsafe {
            if Self.byte_array.as_ptr() != buffer.as_ptr() {
                Self.byte_array = *std::slice::from_raw_parts(buffer.as_ptr(), buffer.len());
            }
        }
        Ok(())
    }

    fn wrap2(&mut self, buffer: Box<dyn DirectBuffer>, offset: usize, length: usize) {
        if SHOULD_BOUNDS_CHECK {
            self.bounds_check_wrap(offset, length, buffer.len())?;
        }
        self.capacity = length;
        self.address_offset = 0 + offset;
        self.wrap_adjustment = offset;

        unsafe {
            if self.byte_array.as_ptr() != buffer.as_ptr() {
                self.byte_array = *std::slice::from_raw_parts(buffer.as_ptr(), buffer.len());
                // self.byte_array = buffer.to_vec()
            }
        }
        Ok(())
    }

    fn wrap3(&mut self, address: usize, length: usize) {
        self.capacity = length;
        self.address_offset = address;
    }

    fn get_long(&self, index: usize) -> i64 {
        todo!()
    }

    fn get_long0(&self, index: usize, byte_order: &dyn ByteOrder) -> i64 {
        todo!()
    }

    fn get_int(&self, index: usize) -> i32 {
        todo!()
    }

    fn get_int0(&self, index: usize, byte_order: &dyn ByteOrder) -> i32 {
        todo!()
    }

    fn parse_natural_int_ascii(&self, index: usize, length: usize) -> i32 {
        todo!()
    }

    fn parse_natural_long_ascii(&self, index: usize, length: usize) -> i64 {
        todo!()
    }

    fn parse_int_ascii(&self, index: usize, length: usize) -> i32 {
        todo!()
    }

    fn parse_long_ascii(&self, index: usize, length: usize) -> i64 {
        todo!()
    }

    fn get_double(&self, index: usize) -> f64 {
        todo!()
    }

    fn get_double0(&self, index: usize, byte_order: &dyn ByteOrder) -> f64 {
        todo!()
    }

    fn get_float(&self, index: usize) -> f32 {
        todo!()
    }

    fn get_float0(&self, index: usize, byte_order: &dyn ByteOrder) -> f32 {
        todo!()
    }

    fn get_short(&self, index: usize) -> i16 {
        todo!()
    }

    fn get_short0(&self, index: usize, byte_order: &dyn ByteOrder) -> i16 {
        todo!()
    }

    fn get_char(&self, index: usize) -> char {
        todo!()
    }

    fn get_char0(&self, index: usize, byte_order: &dyn ByteOrder) -> char {
        todo!()
    }

    fn get_byte(&self, index: usize) -> u8 {
        todo!()
    }

    fn get_bytes(&self, index: usize, dst: &[u8]) {
        todo!()
    }

    fn get_bytes0(&self, index: usize, dst: &[u8], offset: usize, length: usize) {
        todo!()
    }

    fn get_bytes1(&self, index: usize, dst_buffer: Box<dyn MutableDirectBuffer>, offset: usize, length: usize) {
        todo!()
    }

    fn get_string_ascii(&self, index: usize) -> str {
        todo!()
    }

    fn get_string_ascii0(&self, byte_order: dyn ByteOrder) -> str {
        todo!()
    }

    fn get_string_without_length_ascii(&self, index: usize, length: usize) -> str {
        todo!()
    }

    fn get_string_utf8(&self, index: usize) -> str {
        todo!()
    }

    fn get_string_utf8_0(&self, index: usize, length: usize) -> str {
        todo!()
    }

    fn get_string_utf8_1(&self, index: usize, byte_order: dyn ByteOrder) -> str {
        todo!()
    }

    fn get_string_without_length_utf8(&self, index: usize, length: usize) -> str {
        todo!()
    }

    fn bounds_check(&self, index: usize, length: usize) {
        todo!()
    }

    fn wrap_adjustment(&self) -> usize {
        self.wrap_adjustment
    }

    fn address_offset(&self) -> usize {
        self.address_offset
    }

    fn byte_array(&self) -> &[u8] {
        &Self.byte_array
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn check_limit(limit: usize) -> Result<(), String> {
        if limit > Self.capacity {
            return Err(format!("limit={} is beyond capacity={}", limit, Self.capacity));
        }
        Ok(())
    }

    // pub unsafe fn put_char_volatile(o: *mut u8, offset: usize, x: char) {
    //     let x_as_u16 = x as u16;
    //     let addr = o.add(offset) as *mut u16;
    //     ptr::write_volatile(addr, x_as_u16);
    //     compiler_fence(Ordering::SeqCst);
    // }
}
