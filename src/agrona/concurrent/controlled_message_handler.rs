use crate::agrona::mutable_direct_buffer::MutableDirectBuffer;

pub enum Action {
    ABORT,
    BREAK,
    COMMIT,
    CONTINUE
}

pub trait ControlledMessageHandler {
    fn on_message(&self, msg_type_id: i32, buffer: Box<dyn MutableDirectBuffer>, index: usize, length: usize) -> Action;
}
