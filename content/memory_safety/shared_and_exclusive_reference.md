---
title: Shared and Exclusive Reference
---

# Shared vs. Exclusive References in Rust

In Rust, there are two kinds of references:

- `&T` — a shared reference: multiple parties can hold one at the same time, but
  mutation is normally forbidden

- `&mut T` — an exclusive reference: only one party can hold it, and mutation is
  allowed
