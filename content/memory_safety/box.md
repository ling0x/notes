---
title: Box
---

All values in Rust are stack allocated by default. Box is basically a pointer to
a heap allocated value. When Box<T> goes out of scope, its destructor is called,
freeing up the memory on the heap.

### Question: when / why to do box in rust?

We use Box whenever we need to put something on the heap and have a pointer to
it. Because by default Rust values live on the stack, and their size must be
known at compile time.

## Use Box<T> when:

- Recursive types - without Box the type woul dhave inifnite size:

```rust
enum List { Cons(i32, Box<List>), Nil }
```

- Types whose size can’t be known at compile time but you need a sized handle:

Trait objects like `Box<dyn Error>` or `Box<dyn MyTrait>` put the unknown‑sized
value on the heap and give you a pointer of known size so functions can return
or store them easily.

- Moving large data efficiently:

If you have a big struct and want to move ownership around without copying all
its bytes on the stack, you can store it in a Box and move just the pointer
(cheap copy).

## Don't need Box

For example, Vec<T>, String, HashMap<K, V>, etc., these already have a pointer,
so dont need Box for them.

[Reference](https://rustwiki.org/en/rust-by-example/std/box.html)
