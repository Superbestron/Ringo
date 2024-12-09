use crate::agrona::mutable_direct_buffer::MutableDirectBuffer;

pub trait MessageHandler {
    fn on_message(&self, msg_type_id: i32, buffer: Box<dyn MutableDirectBuffer>, index: usize, length: usize);
}
