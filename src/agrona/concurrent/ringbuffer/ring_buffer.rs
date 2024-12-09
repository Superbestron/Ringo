use crate::agrona::concurrent::atomic_buffer::AtomicBuffer;
use crate::agrona::concurrent::controlled_message_handler::ControlledMessageHandler;
use crate::agrona::concurrent::message_handler::MessageHandler;
use crate::agrona::direct_buffer::DirectBuffer;

pub const PADDING_MSG_TYPE_ID: i32 = 1;
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
    fn write(&self, msg_type_id: i32, src_buffer: Box<dyn DirectBuffer>, offset: usize, length: usize) -> bool;

    fn try_claim(&self, msg_type_id: i32, length: usize) -> usize;

    fn commit(&self, index: usize);

    fn abort(&self, index: usize);

    fn read(&self, handler: &dyn MessageHandler) -> i32;

    fn read0(&self, handler: &dyn MessageHandler, message_count_limit: i32) -> i32;

    fn controlled_read(&self, handler: &dyn ControlledMessageHandler) -> i32;

    fn controlled_read0(&self, handler: &dyn ControlledMessageHandler, message_count_limit: i32) -> i32;

    fn max_msg_length(&self) -> i32;

    fn next_correlation_id(&self) -> i64;

    fn buffer(&self) -> &dyn AtomicBuffer; // &dyn because buffer should live long enough

    fn put_consumer_heartbeat_time(&self, time: i64);

    fn consumer_heartbeat_time(&self) -> i64;

    fn producer_position(&self) -> i64;

    fn consumer_position(&self) -> i64;

    fn size(&self) -> i32;

    fn unblock(&self) -> bool;
}
