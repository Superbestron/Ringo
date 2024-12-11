use crate::bit_util::SIZE_OF_LONG;

pub const ALIGNMENT: i32 = SIZE_OF_LONG;
pub const STRICT_ALIGNMENT_CHECKS_PROP_NAME: &str = "agrona.strict.alignment.checks"; // case by case
// boolean STRICT_ALIGNMENT_CHECKS = !SystemUtil.isX64Arch() ||
// "true".equals(SystemUtil.getProperty(STRICT_ALIGNMENT_CHECKS_PROP_NAME, "false"));

pub trait AtomicBuffer {
    fn verify_alignment(&self);
    fn get_long_volatile(&self, index: i32) -> i64;
    fn put_long_volatile(&self, index: i32, value: i64);
    fn put_long_ordered(&mut self, index: i32, value: i64);
    fn add_long_ordered(&self, index: i32, increment: i64);
    fn compare_and_set_long(&self, index: i32, expected_value: i64, update_value: i64);
    fn get_and_set_long(&self, index: i32, value: i64) -> i64;
    fn get_and_add_long(&self, index: i32, delta: i64) -> i64;
    fn get_int_volatile(&self, index: i32) -> i32;
    fn put_int_volatile(&mut self, index: i32, value: i32);
    fn put_int_ordered(&mut self, index: i32, value: i32);
    fn add_int_ordered(&mut self, index: i32, increment: i32);
    fn compare_and_set_int(&mut self, index: i32, expected_value: i32, update_value: i32) -> Result<bool, String>;
    fn get_and_set_int(&self, index: i32, value: i32) -> i32;
    fn get_and_add_int(&self, index: i32, delta: i32) -> i32;
    fn get_short_volatile(&self, index: i32) -> i16;
    fn put_short_volatile(&self, index: i32, value: i16);
    fn get_char_volatile(&self, index: i32) -> char;
    fn put_char_volatile(&self, index: i32, value: char);
    fn get_byte_volatile(&self, index: i32) -> u8;
    fn put_byte_ordered(&self, index: i32, value: u8);
}
