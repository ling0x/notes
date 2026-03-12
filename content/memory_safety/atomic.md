---
title: Atomic
---

In software engineering, **atomic** comes from the Greek word _atomos_, meaning
**indivisible** — and that's exactly what it means in code. An atomic operation
is one that either fully completes or hasn't happened at all; it can never be
observed in a half-finished state by another thread.[^3][^5]

## Why It Matters in Concurrency

Without atomicity, even a simple operation like `x += 1` compiles down to
multiple CPU instructions (load, add, store), which can be interrupted mid-way
by another thread. This creates **race conditions** where two threads read the
same value, both increment it, and one update is silently lost. Atomic
operations solve this by guaranteeing the entire read-modify-write cycle happens
as one uninterruptible unit.[^5][^6]

On a multi-core CPU, when a core begins an atomic operation, it pauses memory
access from other cores for that location until the operation completes.[^4]

## Atomics in Rust

Rust exposes atomic operations through the `std::sync::atomic` module. The types
all start with `Atomic`, for example:[^15]

- `AtomicBool` — atomic boolean
- `AtomicI32`, `AtomicUsize` — atomic integers
- `AtomicPtr` — atomic pointer

Common operations on these types include:[^3]

- **`load`** — reads the value atomically
- **`store`** — writes a value atomically
- **`fetch_add`** — atomically increments and returns the old value
- **`compare_exchange`** — atomically checks if a value matches an expectation,
  and only then replaces it

## Memory Ordering

Every atomic operation in Rust requires an `Ordering` argument (e.g., `Relaxed`,
`Acquire`, `Release`, `SeqCst`). This controls how the CPU and compiler may
reorder instructions around the atomic operation relative to other memory
accesses — a subtle but critical detail for correctness in concurrent code. Rust
inherits this memory model from C++20.[^9][^10]

## Role in the Ecosystem

Atomic types are the **lowest-level building block** for concurrency in Rust.
Higher-level primitives like `Mutex` and `RwLock` are themselves implemented
using atomic operations under the hood.[^3]
<span style="display:none">[^1][^11][^12][^13][^14][^2][^7][^8]</span>

<div align="center">⁂</div>

[^1]: https://doc.rust-lang.org/std/sync/atomic/

[^2]: https://doc.rust-lang.org/nomicon/atomics.html

[^3]: https://marabos.nl/atomics/atomics.html

[^4]: https://leapcell.io/blog/rust-atomics-explained

[^5]: https://whenderson.dev/blog/implementing-atomics-in-rust/

[^6]: https://stackoverflow.com/questions/53587866/what-is-the-difference-between-this-atomic-rust-code-and-its-non-atomic-coun

[^7]: https://www.reddit.com/r/rust/comments/hskm11/having_a_hard_time_understanding_atomic/

[^8]: http://blog.rustbr.org/entendendo-atomicos/

[^9]: https://cfsamsonbooks.gitbook.io/explaining-atomics-in-rust

[^10]: https://www.youtube.com/watch?v=rMGWeSjctlY

[^11]: https://blog.rustbr.org/en/understanding-atomics/

[^12]: https://www.reddit.com/r/rust/comments/1ksqo9i/mastering_rust_atomic_types_a_guide_to_safe/

[^13]: https://dev.to/leapcell/rust-concurrency-atomic-explained-58cl

[^14]: https://rust-lang.guide/guide/learn-async-rust/rust-atomics-and-locs.html

[^15]: https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/sync/atomic/index.html
