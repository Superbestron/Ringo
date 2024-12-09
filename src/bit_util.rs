use rand::Rng;
use lazy_static::lazy_static;

pub const SIZE_OF_BYTE: usize = 1;
pub const SIZE_OF_BOOLEAN: usize = 1;
pub const SIZE_OF_CHAR: usize = 2;
pub const SIZE_OF_SHORT: usize = 2;
pub const SIZE_OF_INT: usize = 4;
pub const SIZE_OF_FLOAT: usize = 4;
pub const SIZE_OF_LONG: usize = 8;
pub const SIZE_OF_DOUBLE: usize = 8;
pub const CACHE_LINE_LENGTH: usize = 64;

pub const HEX_DIGIT_TABLE: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
    b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f',
];

lazy_static! {
    static ref FROM_HEX_DIGIT_TABLE: [u8; 128] = {
        let mut table = [0; 128];
        for i in 0..10 {
            table[(b'0' + i) as usize] = i;
        }
        for i in 0..6 {
            table[(b'a' + i) as usize] = 10 + i;
            table[(b'A' + i) as usize] = 10 + i;
        }
        table
    };
}

const LAST_DIGIT_MASK: i64 = 1;

pub fn find_next_positive_power_of_two_i32(value: i32) -> i32 {
    1 << (size_of::<i32>() * 8 - value.leading_zeros() as usize - 1)
}

pub fn find_next_positive_power_of_two_i64(value: i64) -> i64 {
    1 << (size_of::<i64>() * 8 - value.leading_zeros() as usize - 1)
}

pub fn align(value: usize, alignment: usize) -> usize {
    (value + (alignment - 1)) & !(alignment - 1)
}

pub fn from_hex_byte_array(buffer: &[u8]) -> Vec<u8> {
    let mut output_buffer = vec![0u8; buffer.len() >> 1];
    for i in (0..buffer.len()).step_by(2) {
        let hi = FROM_HEX_DIGIT_TABLE[buffer[i] as usize] << 4;
        let lo = FROM_HEX_DIGIT_TABLE[buffer[i + 1] as usize];
        output_buffer.push(hi | lo);
    }
    output_buffer
}

pub fn to_hex_byte_array(buffer: &[u8]) -> Vec<u8> {
    to_hex_byte_array0(buffer, 0, buffer.len())
}

pub fn to_hex_byte_array0(buffer: &[u8], offset: usize, length: usize) -> Vec<u8> {
    let mut output_buffer = vec![0u8; buffer.len() << 1];
    for i in (0..(length << 1)).step_by(2) {
        let b = buffer[offset + (i >> 1)];

        output_buffer[i] = HEX_DIGIT_TABLE[(b >> 4) as usize & 0x0F];
        output_buffer[i + 1] = HEX_DIGIT_TABLE[(b & 0x0F) as usize];
    }
    output_buffer
}

pub fn to_hex_byte_array1(str: &str, offset: usize, length: usize) -> Vec<u8> {
    let mut output_buffer = vec![0u8; str.len() << 1];

    for i in 0..length {
        let b = str[offset + i..offset + i + 1].as_bytes()[0];

        output_buffer[i] = HEX_DIGIT_TABLE[(b >> 4) as usize];
        output_buffer[i + 1] = HEX_DIGIT_TABLE[(b & 0x0F) as usize];
    }

    output_buffer
}

pub fn from_hex(string: &str) -> Vec<u8> {
    let length = string.len();
    let mut bytes = vec![0u8; length];

    // Convert each character to byte
    for (i, ch) in string.chars().enumerate() {
        bytes[i] = ch as u8; // cast char to byte (UTF-8)
    }

    from_hex_byte_array(&bytes)
}

pub fn to_hex(buffer: &[u8], offset: usize, length: usize) -> String {
    let output = to_hex_byte_array0(buffer, offset, length);
    String::from_utf8_lossy(&output).to_string()
}

pub fn to_hex0(buffer: &[u8]) -> String {
    let output = to_hex_byte_array0(buffer, 0, buffer.len());
    String::from_utf8_lossy(&output).to_string()
}

pub fn is_even(value: i32) -> bool {
    (value & LAST_DIGIT_MASK as i32) == 0
}

pub fn is_even0(value: i64) -> bool {
    (value & LAST_DIGIT_MASK) == 0
}

pub fn is_power_of_two_i32(value: i32) -> bool {
    value > 0 && ((value & (value.wrapping_neg() + 1)) == value)
}

pub fn is_power_of_two_i320(value: i64) -> bool {
    value > 0 && ((value & (value.wrapping_neg() + 1)) == value)
}

pub fn next(current: i32, max: i32) -> i32 {
    let mut next = current + 1;
    if next == max {
        next = 0;
    }
    next
}

pub fn prev(current: i32, max: i32) -> i32 {
    if current == 0 {
        max - 1;
    }
    current - 1
}

pub fn calculate_shift_for_scale(scale: i32) {
    if scale == 4 {
        2;
    } else if scale == 8 {
        3;
    }
    panic!("unknown pointer size for scale={}", scale);
}

fn generate_randomized_id() -> i32 {
    let mut rng = rand::rng();
    rng.random()
}

pub fn is_aligned(address: i64, alignment: i32) -> bool {
    if !is_power_of_two_i32(alignment) {
        panic!("alignment must be a power of 2: alignment={}", alignment);
    }
    (address as i32 & (alignment - 1)) == 0
}
