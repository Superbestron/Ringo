use crate::agrona::concurrent::unsafe_buffer::UnsafeBuffer;
use crate::bit_util::SIZE_OF_INT;

const STR_HEADER_LEN: i32 = SIZE_OF_INT;

pub trait DirectBuffer {
    fn address_offset(&self) -> i32;
    fn byte_array(&self) -> *mut u8;
    fn capacity(&self) -> i32;
    fn check_limit(&self, limit: i32) -> Result<(), String>;
    fn get_long(&self, index: i32) -> i64;
    fn get_int(&self, index: i32) -> i32;
    fn parse_natural_int_ascii(&self, index: i32, length: i32) -> i32;
    fn parse_natural_long_ascii(&self, index: i32, length: i32) -> i64;
    fn parse_int_ascii(&self, index: i32, length: i32) -> i32;
    fn parse_long_ascii(&self, index: i32, length: i32) -> i64;
    fn get_double(&self, index: i32) -> f64;
    fn get_float(&self, index: i32) -> f32;
    fn get_short(&self, index: i32) -> i16;
    fn get_char(&self, index: i32) -> char;
    // fn get_byte(&self, index: i32) -> u8;
    // fn get_bytes(&self, index: i32, dst: &[u8]);
    // fn get_bytes0(&self, index: i32, dst: &[u8], offset: i32, length: i32);
    // fn get_bytes1(&self, index: i32, dst_buffer: &UnsafeBuffer, offset: i32, length: i32);
    // fn get_string_ascii(&self, index: i32) -> str;
    // fn get_string_without_length_ascii(&self, index:i32, length: i32) -> str;
    // fn get_string_utf8(&self, index:i32) -> str;
    // fn get_string_utf8_0(&self, index:i32, length: i32) -> String;
    // fn get_string_without_length_utf8(&self, index:i32, length: i32) -> str;
    // fn bounds_check(&self, index: i32, length: i32);
    fn wrap_adjustment(&self) -> i32;
    fn is_expandable(&self) -> bool;
    fn set_memory(&self, index: i32, length: i32, value: u8);
    fn put_long(&mut self, index: i32, value: i64);
    fn put_int(&mut self, index: i32, value: i32);
    fn put_int_ascii(&self, index: i32, value: i32) -> i32;
    fn put_natural_int_ascii(&self, index: i32, value: i32) -> i32;
    fn put_natural_padding_int_ascii(&self, index: i32, length: i32, value: i64); // throws NumberFormatException
    fn put_natural_int_ascii_from_end(&self, value: i32, end_exclusive: i32) -> i32;
    fn put_natural_long_ascii(&self, index: i32, value: i64) -> i32;
    fn put_long_ascii(&self, index: i32, value: i64) -> i32;
    fn put_double(&self, index: i32, value: f64);
    fn put_float(&self, index: i32, value: f32);
    fn put_short(&self, index: i32, value: i16);
    fn put_char(&self, index: i32, value: char);
    fn put_byte(&self, index: i32, value: u8);
    fn put_bytes(&self, index: i32, bytes: &UnsafeBuffer);
    // Im thinking of just screwing this, since rust doesn't have java's equivalent
    // fn put_bytes1(&self, index: i32, src_buffer: ByteBuffer, offset: i32, bytes: &[u8]);
    fn put_bytes2(&self, index: i32, src_buffer: &UnsafeBuffer, offset: i32, length: i32);
    fn put_string_ascii(&self, index: i32, value: &str) -> i32;
    fn put_string_without_length_ascii(&self, index: i32, value: &str) -> i32;
    fn put_string_without_length_ascii0(&self, index: i32, value: &str, value_offset: i32, length: i32) -> i32;
    fn put_string_utf8(&self, index: i32, value: &str) -> i32;
    fn put_string_utf8_1(&self, index: i32, value: &str, max_encoded_length: i32) -> i32;
    fn put_string_without_length_utf8(&self, index: i32, value: &str) -> i32;
}
