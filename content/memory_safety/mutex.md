---
title: Mutex
---

Mutex is the most commonly used tool for sharing (mutable) data between threads.
Mutex is short for "Mutural Exclusion".

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

## Reference

[Spinlock Considered Harmful](https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html)
