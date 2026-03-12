---
title: Lifetime
---

# static

### Question: Whats the difference between 'static reference and 'static trait bound?

The two uses of 'static in Rust look similar but mean very different things:

- ``&'static T` — A Static Reference:

A &'static T is a reference that lives for the entire duration of the program.
The data it points to must be stored somewhere that never gets freed — typically
in the binary itself (e.g., string literals) or in a static variable.
[rust-lang](https://internals.rust-lang.org/t/idea-aliasing-the-static-lifetime-for-lifetime-parameters-trait-bounds-e-g-auto/19117)

```rust
let s: &'static str = "hello world"; // baked into the binary
```

The key constraint here is on the reference itself: the pointed-to data must
outlive everything.
[rust-lang](https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html)

- `T: 'static` — A Static Trait Bound:

A T: 'static bound does not mean T is a reference at all. It means the type T
contains no non-static references — i.e., T holds no borrowed data that could
expire.

```rust
fn generic<T: 'static>(x: T) { ... }
```

Crucially, any fully owned type (like String, Vec<i32>, u32) automatically
satisfies T: 'static, because it has no internal borrowed references that could
go stale. A &'static str also satisfies it, but a &'a str (with a non-static
lifetime) does not.
[rust-lang](https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html)

### Question: When `T: 'static` Is Needed?

The T: 'static bound is commonly required when values must escape their creation
scope, such as when spawning threads with thread::spawn. Since a spawned thread
can outlive the caller, Rust requires that anything sent into it contains no
short-lived references — which is exactly what T: 'static guarantees.
[Rust Traits are not interfaces](https://www.jamessturtevant.com/posts/rust-traits-are-not-interfaces-and-a-little-on-lifetimes/),
[Learning Rust: static trait bounds](https://codeandbitters.com/static-trait-bound/)

```rust
// thread::spawn requires T: Send + 'static
fn run_in_background<T: Send + 'static>(val: T) {
    std::thread::spawn(move || { /* use val */ });
}
```

A useful mental shortcut: &'static T is about where data lives; T: 'static is
about whether a type is safe to keep indefinitely.
