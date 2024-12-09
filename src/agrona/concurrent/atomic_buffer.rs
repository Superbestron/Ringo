use crate::agrona::mutable_direct_buffer::MutableDirectBuffer;
use crate::bit_util::SIZE_OF_LONG;

pub const ALIGNMENT: usize = SIZE_OF_LONG;
pub const STRICT_ALIGNMENT_CHECKS_PROP_NAME: &str = "agrona.strict.alignment.checks"; // case by case
// boolean STRICT_ALIGNMENT_CHECKS = !SystemUtil.isX64Arch() ||
// "true".equals(SystemUtil.getProperty(STRICT_ALIGNMENT_CHECKS_PROP_NAME, "false"));

pub trait AtomicBuffer: MutableDirectBuffer {
    fn verify_alignment(&self);
    fn get_long_volatile(&self, index: usize) -> i64;
    fn put_long_volatile(&self, index: usize, value: i64);
    fn put_long_ordered(&self, index: usize, value: i64);
    fn add_long_ordered(&self, index: usize, increment: i64);
    fn compare_and_set_long(&self, index: usize, expected_value: i64, update_value: i64);
    fn get_and_set_long(&self, index: usize, value: i64) -> i64;
    fn get_and_add_long(&self, index: usize, delta: i64) -> i64;
    fn get_int_volatile(&self, index: usize) -> i32;
    fn put_int_volatile(&self, index: usize, value: i32);
    fn put_int_ordered(&self, index: usize, value: i32);
    fn add_int_ordered(&self, index: usize, increment: i32);
    fn compare_and_set_int(&self, index: usize, expected_value: i32, update_value: i32);
    fn get_and_set_int(&self, index: usize, value: i32) -> i32;
    fn get_and_add_int(&self, index: usize, delta: i32) -> i32;
    fn get_short_volatile(&self, index: usize) -> i16;
    fn put_short_volatile(&self, index: usize, value: i16);
    fn get_char_volatile(&self, index: usize) -> char;
    fn put_char_volatile(&self, index: usize, value: char);
    fn get_byte_volatile(&self, index: usize) -> u8;
    fn put_byte_ordered(&self, index: usize, value: u8);
}
