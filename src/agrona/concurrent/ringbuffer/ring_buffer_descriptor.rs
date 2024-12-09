use crate::bit_util::CACHE_LINE_LENGTH;

pub const TAIL_POSITION_OFFSET: usize = 0;

/// Offset within the trailer for where the head cache value is stored.
pub const HEAD_CACHE_POSITION_OFFSET: usize = TAIL_POSITION_OFFSET + (CACHE_LINE_LENGTH * 2);

/// Offset within the trailer for where the head value is stored.
pub const HEAD_POSITION_OFFSET: usize = HEAD_CACHE_POSITION_OFFSET + (CACHE_LINE_LENGTH * 2);

/// Offset within the trailer for where the correlation counter value is stored.
pub const CORRELATION_COUNTER_OFFSET: usize = HEAD_POSITION_OFFSET + (CACHE_LINE_LENGTH * 2);

/// Offset within the trailer for where the consumer heartbeat time value is stored.
pub const CONSUMER_HEARTBEAT_OFFSET: usize = CORRELATION_COUNTER_OFFSET + (CACHE_LINE_LENGTH * 2);

/// Total length of the trailer in bytes.
pub const TRAILER_LENGTH: usize = CONSUMER_HEARTBEAT_OFFSET + (CACHE_LINE_LENGTH * 2);

pub fn check_capacity(capacity: usize, min_capacity: usize) -> usize {
    let data_capacity = capacity - TRAILER_LENGTH;
    if !data_capacity.is_power_of_two() {
        panic!(
            "capacity must be a positive power of 2 + TRAILER_LENGTH: capacity={}",
            capacity
        );
    }
    if data_capacity < min_capacity {
        panic!(
            "insufficient capacity: minCapacity={}, capacity={}",
            min_capacity + TRAILER_LENGTH,
            capacity
        );
    }
    data_capacity
}
