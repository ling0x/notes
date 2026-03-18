---
title: "Exercise: Spinlock"
tags: [exercise, rust, concurrency, memory-safety]
---

A from-scratch implementation of a [spinlock](memory_safety/spinlock.md) in Rust
using atomics and `UnsafeCell`. The goal is to understand how a mutex works at
the lowest level — without relying on the OS scheduler.

> [!info] Source Code:
> [exercises/spinlock/src/main.rs](https://github.com/ling0x/notes/blob/main/content/exercises/spinlock/src/main.rs)
