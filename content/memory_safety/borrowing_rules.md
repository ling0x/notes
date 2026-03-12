---
title: Borrowing Rules
---

Rust enforces two hard rules at compile time:

1. You can have any number of immutable references (&T) at the same time

2. You can have only one mutable reference (&mut T) at a time — and when a
   mutable reference is active, no immutable references may exist either

### String example:

#### Question: Why does rust only allow one muatble reference at a time?

A String in Rust is heap-allocated and stores a pointer, a length, and a
capacity internally. If you hold an immutable reference r1 to a String and then
mutate it via a second mutable reference r2 — say by pushing characters — the
String may reallocate its internal buffer to a new heap address. At that point,
r1 would be pointing to freed memory, which is a dangling pointer and causes
undefined behavior. Rust prevents this entirely at compile time.

```rust
let mut s = String::from("hello");

let r1 = &s;      // ✅ immutable borrow
let r2 = &s;      // ✅ another immutable borrow — fine!
let r3 = &mut s;  // ❌ compile error: can't borrow mutably while r1/r2 exist
```

### Furthermore:

The single mutable reference rule actually solves several classes of bugs
simultaneously: ​

- Dangling pointers — mutation causes reallocation, invalidating old references
  (your example)

- Data races — two threads mutating the same memory simultaneously leads to
  unpredictable results

- [Iterator invalidation](iterator_invalidation.md) — modifying a collection
  while iterating over it (a common bug in C++ and Java)

- Compiler optimisation safety — the compiler can safely optimise and even
  vectorise (SIMD) code because it knows no two mutable aliases can overlap

## Mutable Reference Lifetime Is Scoped

The borrow checker is smart enough to track when a reference's last use is, not
just its scope. This means a mutable reference can be created once the immutable
ones are no longer actively used: ​

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
println!("{r1}, {r2}"); // r1 and r2 last used here — they're effectively dropped

let r3 = &mut s; // ✅ now safe, no active immutable refs
r3.push_str(", world");
```

This feature is called
[Non-Lexical Lifetimes (NLL)](/memory_safety/non_lexical_lifetimes.md) and was
introduced to make Rust's borrow checker less restrictive while keeping it safe.

## References:

[Why Does Rust Enforce the “One Mutable or Many Immutable References” Rule in Single-Threaded Programs?](https://users.rust-lang.org/t/why-does-rust-enforce-the-one-mutable-or-many-immutable-references-rule-in-single-threaded-programs/121017/2)

[The Problem With Single-threaded Shared Mutability](https://manishearth.github.io/blog/2015/05/17/the-problem-with-shared-mutability/)
