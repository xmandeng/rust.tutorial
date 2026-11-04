//! Chapter 14: threads, shared state, channels, and async.

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub fn run() {
    // 1. Threads run in parallel on separate CPU cores; there is no GIL.
    //    `move` gives each thread its own copy of `chunk`.
    let numbers: Vec<u64> = (1..=1_000).collect();
    let mut handles = Vec::new();
    for chunk in numbers.chunks(250) {
        let chunk = chunk.to_vec();
        handles.push(thread::spawn(move || chunk.iter().sum::<u64>()));
    }
    let mut total = 0;
    for handle in handles {
        total += handle.join().unwrap(); // join waits for the thread to finish
    }
    println!("sum of 1..=1000 over 4 threads: {total}");

    // 2. Shared, changing state: Arc (shared ownership across threads)
    //    around a Mutex (one thread at a time).
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for _ in 0..8 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1_000 {
                *counter.lock().unwrap() += 1;
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!(
        "counter after 8 threads x 1000: {}",
        *counter.lock().unwrap()
    );

    // 3. Channels: threads send messages instead of sharing memory
    let (sender, receiver) = mpsc::channel();
    for id in 1..=3 {
        let sender = sender.clone();
        thread::spawn(move || {
            sender.send(id * 10).unwrap();
        });
    }
    drop(sender); // close our copy so the loop below ends after the last message
    let mut received: Vec<i32> = receiver.iter().collect();
    received.sort();
    println!("received {received:?}");

    // 4. Async needs a runtime, which Rust does not include. Tokio is the
    //    usual choice. block_on is Python's asyncio.run.
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(fetch_both());
}

// An async fn returns a Future: work that runs only when awaited.
async fn fetch(name: &str, millis: u64) -> String {
    tokio::time::sleep(Duration::from_millis(millis)).await;
    format!("{name} ({millis} ms)")
}

async fn fetch_both() {
    let start = Instant::now();
    // join! runs both futures at once (Python: asyncio.gather)
    let (a, b) = tokio::join!(fetch("prices", 200), fetch("news", 100));
    println!("got {a} and {b}");
    let waited = start.elapsed().as_millis();
    println!("waited under 300 ms, not 200 + 100: {}", waited < 300);
}
