---
title: Arc
---

In Rust, Arc is a smart pointer type that provides thread-safe, shared ownership
of data on the heap. It’s used when multiple threads need to own the same value
at the same time.

## Core idea

- Arc stands for Atomically Reference Counted.

- Type: std::sync::Arc<T>.

- Cloning an Arc<T> creates another handle to the same data and increments an
  internal atomic reference count.

- When the last Arc<T> for a value is dropped, the data is deallocated.

### Question: What "Atomically" actually mean in Arc?

The "atomically" refers specifically to how the reference count itself is
incremented and decremented, not to blocking other threads from reading.

An atomic operation is a CPU-level instruction that completes entirely as a
single, indivisible step, with no possibility of another thread observing it in
a half-finished state. So when two threads clone or drop an Arc simultaneously,
the reference count updates cannot interleave or corrupt each other.

### Question: Does it mean that "another thread cannot read [the reference count] while one thread is reading it."?

No, this is the description of a mutex/lock, not an atomic operation.

The key distinction between Arc and Mutex/Lock:

| Mechanism            | How it works                                                       | Blocking?                           |
| :------------------- | :----------------------------------------------------------------- | :---------------------------------- |
| **Mutex**            | Only one thread accesses data at a time; others wait               | Yes — threads are blocked           |
| **Atomic operation** | The CPU guarantees the operation is indivisible; no waiting needed | No — threads don't block each other |

Atomic operations use special CPU instructions (like `fetch_add` and
`fetch_sub`) that make the increment/decrement happen in one uninterruptible
step. Multiple threads can operate concurrently — they just can't _partially_
observe each other's changes.[^3]

### Question: What Arc does NOT guarantee?

Arc only makes the **reference count** thread-safe, it does not make the
underlying datat T thread-safe. That's why is you need multiple thread to mutate
the shared data, you still need `Arc<Mutex<T>>` or `Arc<RwLock<T>>`
