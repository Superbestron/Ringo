use crate::agrona::concurrent::controlled_message_handler::Action;
use crate::agrona::concurrent::unsafe_buffer::UnsafeBuffer;

pub const PADDING_MSG_TYPE_ID: i32 = -1;
pub const INSUFFICIENT_CAPACITY: i32 = -2;

pub trait RingBuffer {

    fn capacity(&self) -> i32;

    /// Non-blocking write of a message to an underlying ring-buffer
    ///
    /// @param msgTypeId type of the message encoding.
    /// @param srcBuffer containing the encoded binary message.
    /// @param offset    at which the encoded message begins.
    /// @param length    of the encoded message in bytes.
    /// @return true if written to the ring-buffer, or false if insufficient space exists.
    /// @throws IllegalArgumentException if the {@code length} is negative or is greater than {@link #maxMsgLength()}.
    fn write(&self, msg_type_id: i32, src_buffer: &UnsafeBuffer, offset: i32, length: i32) -> bool;

    fn try_claim(&self, msg_type_id: i32, length: i32) -> i32;

    fn commit(&self, index: i32);

    fn abort(&self, index: i32);

    fn read<F>(&self, func: F) -> i32 where F: FnMut(i32, &UnsafeBuffer, i32, i32);

    fn read0<F>(&self, func: F, message_count_limit: i32) -> i32 where F: FnMut(i32, &UnsafeBuffer, i32, i32);

    fn controlled_read<F>(&self, func: F) -> i32 where F: Fn(i32, &UnsafeBuffer, i32, i32) -> Action;

    fn controlled_read0<F>(&self, func: F, message_count_limit: i32) -> i32 where F: Fn(i32, &UnsafeBuffer, i32, i32) -> Action;

    fn max_msg_length(&self) -> i32;

    fn next_correlation_id(&self) -> i64;

    fn buffer(&self) -> &mut UnsafeBuffer;

    fn put_consumer_heartbeat_time(&self, time: i64);

    fn consumer_heartbeat_time(&self) -> i64;

    fn producer_position(&self) -> i64;

    fn consumer_position(&self) -> i64;

    fn size(&self) -> i32;

    fn unblock(&self) -> bool;
}
