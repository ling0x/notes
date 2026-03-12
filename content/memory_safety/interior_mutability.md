---
title: Interior Mutability
---

Interior mutability in Rust is a pattern that lets you change the internal state
of a value through a shared (immutable) reference, something that normal
borrowing rules do not allow.

## Core idea

- Normally, &T means “shared, read-only” and &mut T means “unique, writable”.

- A type has interior mutability if you can call methods that mutate its
  internal data even when you only have &self.

- This is implemented by special wrapper types that enforce safety at runtime
  instead of purely at compile time (often via unsafe inside but a safe public
  API).

## Common types used

- Rust’s standard library provides several types that rely on interior
  mutability:

- Cell<T> and RefCell<T> in std::cell for single-threaded code.

- Mutex<T> and RwLock<T> in std::sync for synchronized mutation across threads.

- Atomic types like AtomicUsize for lock-free concurrent mutation.

## Example

A simple example is a struct that keeps a usage counter even when you only pass
around

```rust
use std::cell::Cell;

struct Counter {
    value: Cell<u32>,
}

impl Counter {
    fn inc(&self) {
        self.value.set(self.value.get() + 1);
    }
}
```

Here, inc takes &self but still mutates the internal value.

[Reference](https://mara.nl/atomics/basics.html#interior-mutability)
