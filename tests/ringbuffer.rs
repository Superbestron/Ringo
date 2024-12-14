#![cfg(not(miri))] // TODO: many assertions failed due to Miri is slow

use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
use std::time::Duration;
use Ringo::agrona::concurrent::ringbuffer::one_to_one_ring_buffer::OneToOneRingBuffer;
use Ringo::agrona::concurrent::unsafe_buffer::UnsafeBuffer;
use Ringo::agrona::concurrent::ringbuffer::ring_buffer::RingBuffer;
use Ringo::agrona::direct_buffer::DirectBuffer;

#[test]
fn test_try_claim() {
    loom::model(|| {
        let buffer = UnsafeBuffer::new(1024);
        let ring_buffer = Arc::new(OneToOneRingBuffer::new(buffer));
        let result = Arc::new(AtomicI32::new(0));
        let mut result_clone = Arc::clone(&result);

        //let mut tmp1 = Arc::clone(&ring_buffer);
        for i in 0..10 {
        //let producer = thread::spawn(|| {
            let index = ring_buffer.try_claim(888, 32);
            if index > 0 {
                let mut buf = ring_buffer.buffer();
                buf.put_int(index + 28, i + 1);
                ring_buffer.commit(index);
            }
        }//);}
        for i in 0..10 {
        //let consumer = thread::spawn(|| {
            ring_buffer.read0(|msg_type, buffer, index, length| {
                let x = buffer.get_int(index + 28);
                println!("{}", x);
                result_clone.store(x, Ordering::SeqCst);
            }, 1);
        }//);}
        //producer.join().unwrap();
        //consumer.join().unwrap();

        // assert!(result.load(Ordering::SeqCst) == 19,
        //         "Unexpected result: {}", result.load(Ordering::SeqCst));
    });
}
