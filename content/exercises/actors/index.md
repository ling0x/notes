---
title: "Exercise: Actors"
tags: [exercise, rust, concurrency, memory-safety]
---

A from-scratch implementation of an
[actor](/concurrency/async_programming/actors.md) in Rust using tokio pmsc
channels. The goal is to understand how an actor works at the lowest level —
without relying on any framework following
[Alice Rhyl's blog post](https://ryhl.io/blog/actors-with-tokio/).

> [!info] Source Code:
> [exercises/actors/src/main.rs](https://github.com/ling0x/notes/blob/main/content/exercises/actors/src/main.rs)
