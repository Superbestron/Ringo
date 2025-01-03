use atomicring::AtomicRingBuffer;
use crossbeam_channel::{bounded, unbounded, Receiver, Sender};
use hdrhistogram::Histogram;
use std::collections::HashSet;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use Ringo::agrona::concurrent::ringbuffer::one_to_one_ring_buffer::OneToOneRingBuffer;
use Ringo::agrona::concurrent::ringbuffer::ring_buffer::RingBuffer;
use Ringo::agrona::concurrent::unsafe_buffer::UnsafeBuffer;
use Ringo::agrona::direct_buffer::DirectBuffer;
use Ringo::bit_util::SIZE_OF_LONG;

const MAX_IN_FLIGHTS: u32 = 1000;

fn new<T>(cap: Option<usize>) -> (Sender<T>, Receiver<T>) {
    match cap {
        None => unbounded(),
        Some(cap) => bounded(cap),
    }
}

fn spsc_chan(cap: Option<usize>) {
    let (tx1, rx1) : (Sender<i64>, Receiver<i64>) = new(cap);
    let (tx2, rx2) : (Sender<i64>, Receiver<i64>) = new(cap);
    // let (tx1, rx1) : (Sender<Box<MyClass>>, Receiver<Box<MyClass>>) = new(cap);
    // let (tx2, rx2) : (Sender<Box<MyClass>>, Receiver<Box<MyClass>>) = new(cap);

    crossbeam::scope(|scope| {
        scope.spawn(|_| {
            loop {
                match rx1.try_recv() {
                    Ok(value) => {
                        loop {
                            match tx2.try_send(value.clone()) {
                                Ok(_) => break,
                                Err(_) => {}
                            }
                        }
                    }
                    Err(_) => {}
                }
            }
        });

        let mut ctr = 0;
        let mut histogram = Histogram::<u64>::new(3).unwrap();
        let mut ori_ms = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

        loop {
            if ctr < MAX_IN_FLIGHTS as usize {
                let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as i64;
                // let msg = Arc::new(MyClass::new(seq, ts));
                // let msg = Box::new(MyClass::new(seq, ts));
                if tx1.try_send(ts).is_ok() {
                    ctr += 1;
                }
            }
            loop {
                match rx2.try_recv() {
                    Ok(value) => {
                        let elapsed = SystemTime::now().duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_nanos() as i64 - value;
                        histogram.record(elapsed as u64).unwrap();
                        ctr -= 1;
                    }
                    Err(_) => break,
                }
            }

            ori_ms = record_time(&mut histogram, ori_ms);
        }
    }).unwrap();
}

fn spsc(cap: usize) {
    let q1 = AtomicRingBuffer::with_capacity(cap);
    let q2 = AtomicRingBuffer::with_capacity(cap);

    crossbeam::scope(|scope| {
        scope.spawn(|_| {
            loop {
                match q1.try_pop() {
                    Some(mut value) => {
                        loop {
                            match q2.try_push(value) {
                                Ok(_) => break,
                                Err(tried) => {value = tried}
                            }
                        }
                    }
                    None => {}
                }
            }
        });

        let mut ctr = 0;
        let mut histogram = Histogram::<u64>::new(3).unwrap();
        let mut ori_ms = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

        loop {
            if ctr < MAX_IN_FLIGHTS as usize {
                let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as i64;
                match q1.try_push(ts) {
                    Ok(_) => {
                        ctr += 1;
                    }
                    Err(_) => {}
                }
            }
            loop {
                match q2.try_pop() {
                    Some(value) => {
                        let elapsed = SystemTime::now().duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_nanos() as i64 - value;
                        histogram.record(elapsed as u64).unwrap();
                        ctr -= 1;
                    }
                    None => break,
                }
            }

            ori_ms = record_time(&mut histogram, ori_ms);
        }
    }).unwrap();
}

fn write(seq: i64, now_ns: i64, buffer: &OneToOneRingBuffer) -> bool {
    let idx = buffer.try_claim(1, 2 * SIZE_OF_LONG);
    // println!("idx: {:?}", idx);
    if idx > 0 {
        let buf = buffer.buffer();
        let mut rb2_offset = idx;
        buf.put_long(rb2_offset, seq);
        rb2_offset += SIZE_OF_LONG;
        buf.put_long(rb2_offset, now_ns);
        buffer.commit(idx);
        return true;
    }
    false
}

fn write0(now_ns: i64, buffer: &OneToOneRingBuffer) -> bool {
    let idx = buffer.try_claim(1, SIZE_OF_LONG);
    // println!("idx: {:?}", idx);
    if idx > 0 {
        let buf = buffer.buffer();
        buf.put_long(idx, now_ns);
        buffer.commit(idx);
        return true;
    }
    false
}

fn spsc_own(cap: usize) {
    let buf1 = UnsafeBuffer::new(cap);
    let buf2 = UnsafeBuffer::new(cap);
    let rb1 = OneToOneRingBuffer::new(buf1);
    let rb2 = OneToOneRingBuffer::new(buf2);

    crossbeam::scope(|scope| {
        scope.spawn(|_| {
            let closure = |msg_type: i32, buffer: &UnsafeBuffer, index: i32, length: i32| {
                // let seq = buffer.get_long(index);
                // let offset = index + SIZE_OF_LONG;
                let ts = buffer.get_long(index);
                loop {
                    if write0(ts, &rb2) {
                        break;
                    }
                }
            };
            loop {
                rb1.read0(closure, 500);
            }
        });

        // let mut set = HashSet::new();
        // let mut seq : i64 = 1;
        let mut ctr = 0;
        let mut histogram = Histogram::<u64>::new(3).unwrap();
        let mut ori_ms = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

        loop {
            // thread::sleep(Duration::from_micros(100000));
            if ctr < MAX_IN_FLIGHTS as usize {
                let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as i64;
                if write0(ts, &rb1) {
                    // set.insert(seq);
                    // println!("Sent: {}", seq);
                    // seq += 1;
                    ctr += 1;
                }
            }
            let closure = |msg_type: i32, buffer: &UnsafeBuffer, index: i32, length: i32| {
                // let seq = buffer.get_long(index);
                // let offset = index + SIZE_OF_LONG;
                let ts = buffer.get_long(index);
                let elapsed = SystemTime::now().duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as i64 - ts;
                histogram.record(elapsed as u64).unwrap();
                // set.remove(&seq);
                ctr -= 1;
                // println!("Received: {}", seq);
            };
            rb2.read0(closure, 500);

            ori_ms = record_time(&mut histogram, ori_ms);
        }
    }).unwrap();
}

fn record_time(histogram: &mut Histogram<u64>, mut ori_ms: u128) -> u128 {
    let now_ms = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    if now_ms - ori_ms > 5_000 {
        ori_ms = now_ms;
        let count = histogram.len();
        let worst = histogram.max();
        let mean = histogram.mean();
        let p99 = histogram.value_at_percentile(99.0);
        let p999 = histogram.value_at_percentile(99.9);
        let p9999 = histogram.value_at_percentile(99.99);
        let output = format!("Latency report generated: total:[{} rounds], worst:[{} ns], avg:[{} ns], p99:[{} ns], p999:[{} ns], p9999:[{} ns]",
                             count, worst, mean, p99, p999, p9999);
        println!("{}", output);
        histogram.reset();
    }
    ori_ms
}

#[tokio::main(flavor = "current_thread")] // single threaded async runtime
async fn main() {
    // spsc_chan(Some(1 << 20));
    // spsc_chan(None);
    // spsc(1 << 20);
    spsc_own(1 << 20);
}
