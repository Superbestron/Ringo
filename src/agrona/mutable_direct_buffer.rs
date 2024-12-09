use byteorder::ByteOrder;
use crate::agrona::direct_buffer::DirectBuffer;

pub trait MutableDirectBuffer: DirectBuffer {
    fn is_expandable(&self) -> bool;
    fn set_memory(&self, index: usize, length: usize, value: u8);
    fn put_long_0(&self, index: usize, value: i64, byte_order: &dyn ByteOrder);
    fn put_long(&self, index: usize, value: i64);
    fn put_int_0(&self, index: usize, value: i64, byte_order: &dyn ByteOrder);
    fn put_int(&self, index: usize, value: i64);
    fn put_int_ascii(&self, index: usize, value: i32) -> usize;
    fn put_natural_int_ascii(&self, index: usize, value: i32) -> usize;
    fn put_natural_padding_int_ascii(&self, index: usize, length: usize, value: i64); // throws NumberFormatException
    fn put_natural_int_ascii_from_end(&self, value: i32, end_exclusive: i32) -> usize;
    fn put_natural_long_ascii(&self, index: usize, value: i64) -> usize;
    fn put_long_ascii(&self, index: usize, value: i64) -> usize;
    fn put_double0(&self, index: usize, value: f64, byte_order: &dyn ByteOrder);
    fn put_double(&self, index: usize, value: f64);
    fn put_float0(&self, index: usize, value: f32, byte_order: &dyn ByteOrder);
    fn put_float(&self, index: usize, value: f32);
    fn put_short0(&self, index: usize, value: i16, byte_order: &dyn ByteOrder);
    fn put_short(&self, index: usize, value: i16);
    fn put_char0(&self, index: usize, value: char, byte_order: &dyn ByteOrder);
    fn put_char(&self, index: usize, value: char);
    fn put_byte(&self, index: usize, value: u8);
    fn put_bytes(&self, index: usize, bytes: &[u8]);
    fn put_bytes0(&self, index: usize, byte_order: &dyn ByteOrder, offset: usize, length: usize);
    // Im thinking of just screwing this, since rust doesn't have java's equivalent
    // fn put_bytes1(&self, index: usize, src_buffer: ByteBuffer, offset: usize, bytes: &[u8]);
    fn put_bytes2(&self, index: usize, src_buffer: Box<dyn DirectBuffer>, offset: usize, bytes: &[u8]);
    fn put_string_ascii(&self, index: usize, value: &str) -> usize;
    fn put_string_ascii0(&self, index: usize, value: &str, byte_order: &dyn ByteOrder) -> usize;
    fn put_string_without_length_ascii(&self, index: usize, value: &str) -> usize;
    fn put_string_without_length_ascii0(&self, index: usize, value: &str, value_offset: usize, length: usize) -> usize;
    fn put_string_utf8(&self, index: usize, value: &str) -> usize;
    fn put_string_utf8_0(&self, index: usize, value: &str, byte_order: &dyn ByteOrder) -> usize;
    fn put_string_utf8_1(&self, index: usize, value: &str, max_encoded_length: usize) -> usize;
    fn put_string_utf8_2(&self, index: usize, value: &str, byte_order: &dyn ByteOrder, max_encoded_length: usize) -> usize;
    fn put_string_without_length_utf8(&self, index: usize, value: &str) -> usize;
}
