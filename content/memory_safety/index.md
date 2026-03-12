---
title: Memory Safety
---

Notes on Rust's memory safety model, covering ownership, borrowing, and
concurrency primitives.

## Topics

- **Ownership & Borrowing** — borrowing rules, non-lexical lifetimes, shared and
  exclusive references, interior mutability
- **Smart Pointers** — `Box`, `Arc`, and the `Drop` trait
- **Concurrency** — `Mutex`, `RwLock`, atomics, spinlocks, `Send`/`Sync` traits,
  and synchronization/concurrency primitives
- **Memory Model** — stack vs. heap, segfaults, iterator invalidation
- **Practical Patterns** — connection pooling, PostgreSQL connections
