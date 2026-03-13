---
title: Channels
---

Channels: A
[communication primitive](/concurrent_programming/concurrency_primitives.md)
used in many languages (Rust, Go, .NET, etc.) to pass messages between
concurrent pieces of code (goroutines, tasks, threads).

# Tokio's channel primitives

- mpsc: multi-producer, single-consumer channel. Many values can be sent.

- oneshot: single-producer, single consumer channel. A single value can be sent.

- broadcast: multi-producer, multi-consumer. Many values can be sent. Each
  receiver sees every value.

- watch: multi-producer, multi-consumer. Many values can be sent, but no history
  is kept. Receivers only see the most recent value.

  [tokio-tutorial](/exercises/tokio-tutorial/index.md)
