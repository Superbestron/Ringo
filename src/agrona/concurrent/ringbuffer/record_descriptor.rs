use crate::bit_util::SIZE_OF_INT;

pub const HEADER_LENGTH: i32 = SIZE_OF_INT << 1;

pub const ALIGNMENT: i32 = HEADER_LENGTH;

pub fn length_offset(record_offset: i32) -> i32 {
    record_offset
}

pub fn type_offset(record_type: i32) -> i32 {
    record_type + SIZE_OF_INT
}

pub fn encoded_msg_offset(record_type: i32) -> i32 {
    record_type + HEADER_LENGTH
}

pub fn check_type_id(msg_type_id: i32) {
    if msg_type_id < 1 {
        panic!("message type id must be greater than zero, msgTypeId={}", msg_type_id)
    }
}
