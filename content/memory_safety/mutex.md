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

## Reference

[Spinlock Considered Harmful](https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html)
