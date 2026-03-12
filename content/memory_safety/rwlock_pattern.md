---
title: RWLock Pattern
---

The RWLock (Readers-Writer Lock) pattern is a concurrency synchronization
primitive that allows multiple threads to read shared data simultaneously, but
requires exclusive access for any write operation. It's ideal for scenarios
where reads are frequent and writes are rare, as it avoids the unnecessary
serialization that a plain Mutex would impose.

## Core Rules

- Multiple readers can hold the lock at the same time — reads are non-exclusive ​

- Only one writer can hold the lock at a time — writes are exclusive ​

- A write lock blocks all readers and other writers until it is released ​

- Once a writer is active, new readers must wait

## Rust RwLock Example

Rust provides std::sync::RwLock<T> in its standard library. Here's a practical
example with multiple reader threads and one writer thread:

```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    // Wrap data in Arc<RwLock<T>> for shared ownership across threads
    let data = Arc::new(RwLock::new(0u32));

    // --- Writer thread: exclusive access ---
    let data_writer = Arc::clone(&data);
    let writer = thread::spawn(move || {
        let mut w = data_writer.write().unwrap(); // blocks until no readers/writers
        *w += 42;
        println!("Writer set value to: {}", *w);
    }); // write lock dropped here

    writer.join().unwrap();

    // --- Multiple reader threads: concurrent access ---
    let reader_handles: Vec<_> = (0..4).map(|i| {
        let data_reader = Arc::clone(&data);
        thread::spawn(move || {
            let r = data_reader.read().unwrap(); // multiple readers can hold this simultaneously
            println!("Reader {} sees value: {}", i, *r);
        })
    }).collect();

    for handle in reader_handles {
        handle.join().unwrap();
    }
}
```

### Output (reader order may vary):

```text
Writer set value to: 42
Reader 0 sees value: 42
Reader 2 sees value: 42
Reader 1 sees value: 42
Reader 3 sees value: 42
```

## When to Use vs. Mutex

RwLock is only better than Mutex when you have many concurrent readers competing
at the same time. For a single reader or low concurrency, an uncontended RwLock
is no faster than a Mutex because readers still acquire the lock and update
internal state. A good rule of thumb: if your workload is read-heavy (e.g., 90%+
reads) with rare writes, RwLock gives a meaningful throughput boost. ​

## Watch Out: Lock Poisoning

In Rust, if a thread panics while holding a write lock, the RwLock becomes
poisoned. Subsequent calls to .read() or .write() will return an Err, which you
handle via .unwrap() or .into_inner(). This is a safety feature to prevent
access to potentially corrupt data.

[RWLock](https://dev-doc.rust-lang.org/beta/std/sync/struct.RwLock.html)
