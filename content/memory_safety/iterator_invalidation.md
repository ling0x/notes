---
title: Iterator Invalidation
---

Iterator invalidation is a bug where an iterator becomes invalid because the
underlying collection it references is modified during iteration. Rust prevents
this at compile time through its borrow checker, unlike languages like C++ where
it causes undefined behaviour or crashes at runtime.

When you iterate over a collection (e.g., a Vec), the iterator holds a reference
to that collection's memory. If the collection is modified — say, by pushing a
new element — the Vec may need to reallocate its memory to grow, freeing the old
memory and leaving the iterator pointing to a dangling address. At best this
causes a crash (segfault); at worst it silently corrupts memory.

Rust's borrow checker enforces that you cannot hold an immutable borrow and a
mutable borrow at the same time. When you call v.iter(), Rust creates an
immutable borrow on the entire collection. Any attempt to call v.push() or
v.remove() during the loop requires a mutable borrow — which the compiler
outright refuses to compile:

```rust
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    for elem in &vec {
        vec.push(elem * 2); // COMPILE ERROR: cannot borrow `vec` as mutable
                            // because it is also borrowed as immutable
    }
}
```

The error is caught at compile time, not at runtime. ​

## What You _Can_ Do Safely

| Pattern                                  | Allowed? | Why                                        |
| :--------------------------------------- | :------- | :----------------------------------------- |
| `v.iter()` + read elements               | ✅       | Multiple immutable borrows are fine        |
| `v.iter_mut()` + modify each element     | ✅       | One mutable borrow, no structural changes  |
| `v.iter()` + `v.push()` in the same loop | ❌       | Immutable + mutable borrow conflict        |
| Collect indices, then modify after loop  | ✅       | Iterator is dropped before mutation begins |

A common safe workaround is to use `retain` or collect the changes you need into
a separate `Vec`, then apply them after the loop ends — by which point the
iterator has been dropped and the borrow is released.

## It's effectively threaded

> Aliasing with mutability in a sufficiently complex, single-threaded program is
> effectively the same thing as accessing data shared across multiple threads
> without a lock

> My intuition is that code far away from my code might as well be in another
> thread, for all I can reason about what it will do to shared mutable state.

[Reference](http://manishearth.github.io/blog/2015/05/17/the-problem-with-shared-mutability/)
