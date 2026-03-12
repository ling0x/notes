---
title: Concurrency Primitives
---

**Concurrency primitives** in Rust are the fundamental building blocks that
allow multiple parts of a program to run simultaneously — safely and without
data races. They are the low-level tools you use to coordinate concurrent tasks,
share data between threads, and synchronize execution.

## Why Rust Is Special Here

Rust's ownership and type system enforce concurrency safety at **compile time**,
not at runtime. Many bugs that would silently corrupt data in other languages
become compile errors in Rust, which is why Rust calls its approach "fearless
concurrency".
[doc.rust-lang](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

## The Core Primitives

- **Threads** — the most basic primitive; independent paths of execution that
  run concurrently, letting you exploit multi-core processors
  [earthly](https://earthly.dev/blog/rust-concurrency-patterns-parallel-programming/)
- **Channels (`mpsc`)** — typed message-passing pipes with a sender and receiver
  handle; one thread sends data, another receives it, avoiding shared memory
  entirely [news.ycombinator](https://news.ycombinator.com/item?id=7851274)
- [**Mutex (`Mutex<T>`)**](/memory_safety/mutex.md) — short for _mutual
  exclusion_; only one thread can access the protected data at a time,
  preventing data races on shared state
  [earthly](https://earthly.dev/blog/rust-concurrency-patterns-parallel-programming/)
- [**Arc (`Arc<T>`)**](/memory_safety/arc.md) — _Atomic Reference Counting_;
  lets multiple threads share ownership of a value safely
  [doc.rust-lang](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
- [**`RwLock<T>`**](/memory_safety/rwlock_pattern.md) — like a Mutex, but allows
  many simultaneous readers or one exclusive writer
- [**Atomic types**](/memory_safety/atomic.md) — low-level primitives (e.g.,
  `AtomicUsize`) for lock-free, thread-safe operations on simple values
  [web.mit](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/concurrency.html)

## Key Traits: `Send` and `Sync`

Rust enforces concurrency rules through two marker traits:
[web.mit](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/concurrency.html)

- **`Send`** — a type can be transferred (moved) to another thread
- **`Sync`** — a type can be safely _referenced_ from multiple threads
  simultaneously

These traits are automatically implemented by the compiler where safe, and
withheld where they aren't — so you can't accidentally send a non-thread-safe
type across a thread boundary.
[google.github](https://google.github.io/comprehensive-rust/concurrency/welcome.html)

## A Simple Mental Model

Think of concurrency primitives as traffic rules for threads. Channels say
_"pass the data by handing it off"_, while `Mutex`/`Arc` say _"share the data,
but take turns"_. Rust's compiler acts as the traffic enforcer, rejecting unsafe
patterns before your code ever runs.
[dzone](https://dzone.com/articles/concurrency-in-rust-safe-and-efficient-code)
