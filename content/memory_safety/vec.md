---
title: Vec
---

# `Vec<u8>`

Getting the size of a Vec<u8> in Rust is straightforward — since each element is
exactly 1 byte, .len() gives you the byte count directly.

## `len()` vs `capacity()`

A `Vec<u8>` internally holds three things: a pointer to heap memory, a length,
and a capacity.

- `len()` — the number of actual bytes of data present. This is the file size.

- `capacity()` — the total memory reserved on the heap, which may be larger than
  len() to avoid frequent reallocations as the vector grows. This is an internal
  memory management detail.

  For example, a `Vec<u8>` with 500 bytes of real data might have a capacity of
  512 — those extra 12 bytes are pre-allocated empty slots Rust reserved
  speculatively. They contain no real data.
  ​[rust-lang](https://doc.rust-lang.org/std/vec/struct.Vec.html)
