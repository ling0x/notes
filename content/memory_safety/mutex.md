---
title: Mutex
---

Mutex is the most commonly used tool for sharing (mutable) data between threads.
Mutex is short for "Mutural Exclusion". It uses UnsafeCell under the hood, and
provides a safe interface MutexGuard for threads to access the data with
"Interior Mutability". It is a thread-safe wrapper around some data T that
ensures only one thread can access that data at a time

```
             +-----------------------------+
             |       Arc<Mutex<T>>         |
             |   (shared across threads)   |
             |                             |
             |   +---------------------+   |
             |   |       Mutex<T>      |   |
             |   |  (lock + the data)  |   |
             |   |                     |   |
             |   |   +-------------+   |   |
             |   |   |   value T   |   |   |
             |   |   +-------------+   |   |
             |   +----------^----------+   |
             +--------------|--------------+
                            |
        clones of Arc       |
   (Arc::clone(&mutex))     |
      sent to threads       |
                            |
         +------------------+------------------+
         |                  |                  |
         v                  v                  v
+----------------+  +----------------+  +----------------+
|   thread 1     |  |   thread 2     |  |   thread 3     |
|                |  |                |  |                |
| lock()         |  | lock()         |  | lock()         |
|   |            |  |   |            |  |   |            |
|   v            |  |   v            |  |   v            |
| MutexGuard<T>  |  | MutexGuard<T>  |  | MutexGuard<T>  |
| (exclusive     |  | (exclusive     |  | (exclusive     |
|  access)       |  |  access)       |  |  access)       |
+----------------+  +----------------+  +----------------+

Only ONE MutexGuard can exist at a time:
- When a thread calls lock(), it blocks until it gets a MutexGuard.
- While a thread holds the guard, others wait.
- When the guard is dropped (goes out of scope), the lock is released.
```

## Basic Example:

```rust
use std::sync::Mutex;

fn main() {
    let my_mutex = Mutex::new(5);

    {
        let mut guard = my_mutex.lock().unwrap();
        *guard = 6;           // modify protected data
    } // guard dropped here, mutex unlocked

    println!("{:?}", my_mutex); // prints: Mutex { data: 6 }
}
```

## Poisoning

- If a thread panics while holding the lock, the mutex becomes poisoned to
  signal that the data may be in an inconsistent state.

- After that, `.lock()` returns an `Err(PoisonError)` instead of
  `Ok(MutexGuard)`; you often handle this with `unwrap()` (propagating the
  panic) or `unwrap_or_else` to recover.

### Question: whats the difference between Arc and Mutex?

Arc and Mutex solve different problems: Arc gives shared ownership of data
across threads, while Mutex controls exclusive access to data (usually for
mutation) so only one thread touches it at a time.

## How Arc and Mutex are used together

A very common pattern in Rust is Arc<Mutex<T>>, where:

- Arc lets many threads share ownership of the same Mutex<T>.

- Mutex ensures that each access to T is exclusive, so concurrent mutation is
  safe.

## Quick example intuition

If you want several threads to read the same configuration that never changes,
you typically use Arc<Config>.

If you want several threads to increment a shared counter, you typically use
Arc<Mutex<u32>> so they can all own the counter and take turns mutating it
safely.

## On using `std::sync::Mutex` and `tokio::sync::Mutex`

> Note that std::sync::Mutex and not tokio::sync::Mutex is used to guard the
> HashMap. A common error is to unconditionally use tokio::sync::Mutex from
> within async code. An async mutex is a mutex that is locked across calls to
> .await.

> A synchronous mutex will block the current thread when waiting to acquire the
> lock. This, in turn, will block other tasks from processing. Switching to
> tokio::sync::Mutex will cause the task to yield control back to the executor,
> but this will usually not help with performance as the asynchronous mutex uses
> a synchronous mutex internally.

> As a rule of thumb, using a synchronous mutex from within asynchronous code is
> fine as long as contention remains low and the lock is not held across calls
> to .await.

### Question: What does “Locked across calls to .await” mean?

It means: you acquired the mutex lock before an `.await`, and you still hold
that lock while the future is suspended at that `.await` and possibly resumed
later on another poll.

In async Rust, every .await is a point where your function can be suspended and
other tasks can run on the same thread.

Code that hold lock across the await:

```rust
async fn f(m: &std::sync::Mutex<Data>) {
    let mut guard = m.lock().unwrap();   // lock acquired here

    do_something_async().await;          // <-- await while still holding `guard`

    guard.value += 1;                    // lock released only when `guard` is dropped
}
```

the Mutex remains locked for the entire time between taking guard and dropping
it, including the whole period when the function is paused at
do_something_async().await. The lock is “held across the await”.
[How to Use async Rust Without Blocking the RUntime](https://oneuptime.com/blog/post/2026-01-07-rust-async-without-blocking/view)

By contrast, code that does not hold the lock across an .await acquires and
releases it entirely before the first .await:

```rust
async fn g(m: &std::sync::Mutex<Data>) {
    {
        let mut guard = m.lock().unwrap();
        guard.counter += 1;
        // lock is dropped at the end of this block
    } // <--- lock released here

    do_something_async().await; // no lock held during await
}
```

Here the critical section is synchronous and short, so using std::sync::Mutex is
fine because the lock is never held while the task is suspended at .await.
[Sync mutex in async program](https://users.rust-lang.org/t/sync-mutex-in-async-program/66118)

A quick way to think about it:

- “Held/locked across `.await`” = the lifetime of your `MutexGuard` spans over
  an `.await` expression.

- “Not held across `.await`” = you drop the guard (end its scope) before you hit
  any `.await`.

However, the `tokio::sync::Mutex` is designed to be held across `await` points:

```rust
async fn g(m: &tokio::sync::Mutex<Data>) {
  let mut guard = m.lock().await;    // yields if needed, no thread block
  do_something_async().await;        // still holding the lock
  guard.counter += 1;
} // lock released when guard is dropped
```

is legal in terms of the runtime: you’re not blocking a worker thread while
waiting for I/O, other tasks can still run.

So “locked across calls to .await” means:

- With std::sync::Mutex: strongly discouraged; can stall threads and deadlock.

- With tokio::sync::Mutex: allowed and supported; the runtime knows how to park
  and wake tasks that are waiting on the async mutex.

#### When to choose which

- Use std::sync::Mutex in async code when:

  - You only lock for very short, synchronous work.

  - You always drop the guard before any .await.

- Use tokio::sync::Mutex when:

  - You really need to keep the lock across one or more .awaits, or

  - The critical section involves async operations that can’t be easily
    refactored to be purely synchronous.

```
## Reference

[Spinlock Considered Harmful](https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html)
```
