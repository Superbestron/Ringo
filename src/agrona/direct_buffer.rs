use byteorder::ByteOrder;
use crate::agrona::mutable_direct_buffer::MutableDirectBuffer;
use crate::bit_util::SIZE_OF_INT;

const STR_HEADER_LEN: usize = SIZE_OF_INT;

pub trait DirectBuffer: PartialOrd {
    fn wrap(&self, buffer: &[u8]);
    fn wrap0(&self, buffer: &[u8], offset: usize, length: usize);
    fn wrap1(&self, buffer: Box<dyn DirectBuffer>);
    fn wrap2(&self, buffer: Box<dyn DirectBuffer>, offset: usize, length: usize);
    fn wrap3(&self, address: usize, buffer: Box<dyn DirectBuffer>);
    fn address_offset(&self) -> usize;
    fn byte_array(&self) -> &[u8];
    fn capacity(&self) -> usize;
    fn check_limit(limit: usize) -> Result<(), String>;
    fn get_long(&self, index: usize) -> i64;
    fn get_long0(&self, index: usize, byte_order: &dyn ByteOrder) -> i64;
    fn get_int(&self, index: usize) -> i32;
    fn get_int0(&self, index: usize, byte_order: &dyn ByteOrder) -> i32;
    fn parse_natural_int_ascii(&self, index: usize, length: usize) -> i32;
    fn parse_natural_long_ascii(&self, index: usize, length: usize) -> i64;
    fn parse_int_ascii(&self, index: usize, length: usize) -> i32;
    fn parse_long_ascii(&self, index: usize, length: usize) -> i64;
    fn get_double(&self, index: usize) -> f64;
    fn get_double0(&self, index: usize, byte_order: &dyn ByteOrder) -> f64;
    fn get_float(&self, index: usize) -> f32;
    fn get_float0(&self, index: usize, byte_order: &dyn ByteOrder) -> f32;
    fn get_short(&self, index: usize) -> i16;
    fn get_short0(&self, index: usize, byte_order: &dyn ByteOrder) -> i16;
    fn get_char(&self, index: usize) -> char;
    fn get_char0(&self, index: usize, byte_order: &dyn ByteOrder) -> char;
    fn get_byte(&self, index: usize) -> u8;
    fn get_bytes(&self, index: usize, dst: &[u8]);
    fn get_bytes0(&self, index: usize, dst: &[u8], offset: usize, length: usize);
    fn get_bytes1(&self, index: usize, dst_buffer: Box<dyn MutableDirectBuffer>, offset: usize, length: usize);
    fn get_string_ascii(&self, index: usize) -> str;
    fn get_string_ascii0(&self, byte_order: dyn ByteOrder) -> str;
    fn get_string_without_length_ascii(&self, index:usize, length: usize) -> str;
    fn get_string_utf8(&self, index:usize) -> str;
    fn get_string_utf8_0(&self, index:usize, length: usize) -> str;
    fn get_string_utf8_1(&self, index:usize, byte_order: dyn ByteOrder) -> str;
    fn get_string_without_length_utf8(&self, index:usize, length: usize) -> str;
    fn bounds_check(&self, index: usize, length: usize);
    fn wrap_adjustment(&self) -> usize;
}
