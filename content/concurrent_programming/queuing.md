---
title: queuing
---

Concurrency and queuing with tokio in rust must be explicitly introduced. Ways
to do this include:

- tokio::spawn
- select!
- join!
- mpsc::channel

When doing so, take care to ensure the total amount of concurrency is bounded.
For example, when writing a TCP accept loop, ensure that the total number of
open sockets is bounded. When using mpsc::channel, pick a manageable channel
capacity. Specific bound values will be application specific.

Taking care and picking good bounds is a big part of writing reliable Tokio
applications.
