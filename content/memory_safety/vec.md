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

```
 Stack                        Heap
─────────────────            ──────────────────────────────────────────────────
┌───────────────┐            ┌─────────────────────────────┬──────────────────┐
│   ptr         │──────────► │  REAL DATA (500 bytes)      │  EMPTY (12 bytes)│
├───────────────┤            │                             │                  │
│   len = 500   │            │  [0x01][0xFF][0x3A]...      │  [ ][ ][ ]...    │
├───────────────┤            │                             │                  │
│   cap = 512   │            │  ◄────── len = 500 ────────►│◄─── cap-len ────►│
└───────────────┘            └─────────────────────────────┴──────────────────┘
                            ◄──────────────── cap = 512 ──────────────────────►
```

- `ptr` points to the start of the heap block ​

- `len = 500` marks how far into the block contains real, valid data — this is
  your file size

- `cap = 512` is the total reserved memory; the trailing 12 bytes are allocated
  but uninitialised and ignored
  [stackoverflow](https://stackoverflow.com/questions/54889521/whats-the-difference-between-len-and-capacity)
  ​

- When you call `.len()`, you get `500` — exactly the bytes fetched from the
  database, nothing more

### Question: Why `512`?

512 was just an illustrative, round-number example, not a special Rust rule.

In reality:

- Rust does not guarantee “500 bytes → capacity 512”. The allocator chooses how
  much memory to give you, and `Vec` typically grows by some factor (often ~2×),
  but the exact value is an implementation detail.

- When a `Vec` needs more space (because `len == capacity` and you push again),
  it reallocates to a larger capacity to reduce how often it has to reallocate
  in the future. That larger capacity might be 512, 640, 1000, etc., depending
  on the previous capacity and the growth strategy.

- So “500 data, 512 capacity” was just to show the idea: **capacity ≥ len**, and
  the extra part is reserved space for future pushes, not real data.

### Question: Whats the difference between "padding" and "pre-allocated empty slot"?

“Padding” is a term we use for extra bytes inserted inside or between fields of
a struct to satisfy alignment requirements of the CPU (e.g., to align a u64 on
an 8‑byte boundary). However, in a `Vec<T>`:

- `len` is how many initialized elements you have.

- `capacity` is how many elements the heap allocation can hold.

- The bytes between `le`n and `capacity` are logically **uninitialized storage
  for future elements**, not alignment padding.

  So for a `Vec<u8>` with `len = 500` and `capacity = 512`, those 12 bytes are
  just unused, reserved space that Rust can fill later if you push more bytes;
  they’re not considered padding in the usual memory-layout sense.
