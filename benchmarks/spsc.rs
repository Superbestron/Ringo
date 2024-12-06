use crossbeam_channel::{bounded, unbounded, Receiver, Sender};
use hdrhistogram::Histogram;
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
struct MyClass {
    seq: i32,
    ts: i64,
}

impl MyClass {
    // Associated function to create a new `MyClass` instance
    fn new(seq: i32, ts: i64) -> Self {
        MyClass {
            seq,
            ts,
        }
    }
}

fn new<T>(cap: Option<usize>) -> (Sender<T>, Receiver<T>) {
    match cap {
        None => unbounded(),
        Some(cap) => bounded(cap),
    }
}

fn spsc(cap: Option<usize>) {
    let max_in_flights = 1000;
    let (tx1, rx1) : (Sender<MyClass>, Receiver<MyClass>) = new(cap);
    let (tx2, rx2) : (Sender<MyClass>, Receiver<MyClass>) = new(cap);
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
                            // match tx2.try_send(value.clone()) {
                            //     Ok(_) => break,
                            //     Err(_) => {}
                            // }
                        }
                    }
                    Err(_) => {}
                }
            }
        });

        let mut set = HashSet::new();
        let mut seq = 1;
        let mut histogram = Histogram::<u64>::new(3).unwrap();
        let mut ori_ms = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

        loop {
            if set.len() < max_in_flights {
                let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as i64;
                let msg = MyClass::new(seq, ts);
                // let msg = Arc::new(MyClass::new(seq, ts));
                // let msg = Box::new(MyClass::new(seq, ts));
                if tx1.try_send(msg.clone()).is_ok() {
                    set.insert(seq);
                    seq += 1;
                }
            }
            loop {
                match rx2.try_recv() {
                    Ok(value) => {
                        let elapsed = SystemTime::now().duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_nanos() as i64 - value.ts;
                        histogram.record(elapsed as u64).unwrap();
                        set.remove(&value.seq);
                    }
                    Err(_) => break,
                }
            }

            let now_ms = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            if now_ms - ori_ms > 5_000 {
                ori_ms = now_ms;
                let count = histogram.len();
                let worst = histogram.max();
                let mean = histogram.mean();
                let p99 = histogram.value_at_percentile(99.0);
                let p999 = histogram.value_at_percentile(99.9);
                let p9999 = histogram.value_at_percentile(99.99);
                let output = format!("Latency report generated: total:[{}rounds], worst:[{}ns], avg:[{}ns], p99:[{}ns], p999:[{}ns], p9999:[{}ns]",
                                     count, worst, mean, p99, p999, p9999);
                println!("{}", output);
                histogram.reset();
            }
        }
    }).unwrap();
}

#[tokio::main(flavor = "current_thread")] // single threaded async runtime
async fn main() {
    spsc(Some(5_000));
    // spsc(None);
}
