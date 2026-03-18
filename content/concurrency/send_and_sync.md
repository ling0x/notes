---
title: Send and Sync
---

`Send` and `Sync` are both **marker traits** in Rust that govern thread safety,
but they address different aspects of how data crosses thread boundaries.

## `Send`: Ownership Transfer

A type is `Send` if it is safe to **transfer ownership** of a value to another
thread. In other words, you can _move_ a `T` into a new thread. Most primitive
types (`i32`, `bool`), `String`, `Vec<T>`, and `Arc<T>` are `Send`. Types that
are **not** `Send` include:[^1][^6]

- `Rc<T>` — its reference count is non-atomic, so moving it to another thread
  risks a data race on the count[^3]
- Raw pointers (`*const T`, `*mut T`) — no synchronization guarantees when
  dereferenced across threads[^6]

## `Sync`: Shared Reference Access

A type is `Sync` if it is safe for **multiple threads to hold shared
references** (`&T`) to it simultaneously. Formally, `T` is `Sync` if and only if
`&T` is `Send`. Types that are **not** `Sync` include:[^5][^1]

- `RefCell<T>` — its borrow-checking is not thread-safe (non-atomic interior
  mutability)[^6]
- `Cell<T>` — same reason; mutable access is not synchronized[^8]
- Raw pointers[^6]

## How They Relate

The two traits are closely intertwined — `Sync` is almost _defined in terms of_
`Send`. A useful mental model:[^8]

| Trait  | Asks…                                            | Example: YES      | Example: NO             |
| :----- | :----------------------------------------------- | :---------------- | :---------------------- |
| `Send` | Can ownership move to another thread?            | `Arc<Mutex<T>>`   | `Rc<T>`                 |
| `Sync` | Can a shared `&T` be used from multiple threads? | `Mutex<T>`, `i32` | `RefCell<T>`, `Cell<T>` |

A type can be `Send` but not `Sync` (e.g., `Mutex<T>` where ownership transfers
are fine but concurrent `&T` access needs the lock), or `Sync` but not `Send`
(e.g., a `MutexGuard`, which can be shared but must be released on the
originating thread). Both traits are **automatically derived** by the compiler —
you only need to intervene when working with `unsafe` code or types that should
explicitly opt out.[^3][^8]
<span style="display:none">[^10][^2][^4][^7][^9]</span>

<div align="center">⁂</div>

[^1]: https://doc.rust-lang.org/nomicon/send-and-sync.html

[^2]: https://www.reddit.com/r/rust/comments/ctdkyr/understanding_sendsync/

[^3]: https://blog.masteringbackend.com/rust-send-and-sync-in-simple-terms

[^4]: https://stackoverflow.com/questions/59428096/understanding-the-send-trait

[^5]: https://www.reddit.com/r/rust/comments/1csrbhf/understanding_send_trait_in_rust/

[^6]: https://leapcell.io/blog/understanding-send-and-sync-in-rust-async-handlers

[^7]: https://masteringbackend.com/posts/rust-send-and-sync-in-simple-terms

[^8]: https://www.youtube.com/watch?v=yOezcP-XaIw

[^9]: https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/nomicon/send-and-sync.html

[^10]: https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html
