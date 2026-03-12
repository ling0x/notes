---
title: Tokio Tutorial
---

https://tokio.rs/tokio/tutorial

## Four types of tokio channels:

### oneshot channel

The oneshot channel supports sending a single value from a single producer to a
single consumer. This channel is usually used to send the result of a
computation to a waiter.

### mpsc channel

The mpsc channel supports sending many values from many producers to a single
consumer. This channel is often used to send work to a task or to receive the
result of many computations.

### broadcast channel

The broadcast channel supports sending many values from many producers to many
consumers. Each consumer will receive each value. This channel can be used to
implement “fan out” style patterns common with pub / sub or “chat” systems.

### watch channel

The watch channel supports sending many values from many producers to many
consumers. However, only the most recent value is stored in the channel.
Consumers are notified when a new value is sent, but there is no guarantee that
consumers will see all values.

https://tokio.rs/tokio/tutorial/channels

https://docs.rs/tokio/1.49.0/tokio/sync/

## Concepts

### Tokio Tasks

Tasks are the unit of execution managed by the scheduler. Spawning the task
submits it to the Tokio scheduler, which then ensures that the task executes
when it has work to do. The spawned task may be executed on the same thread as
where it was spawned, or it may execute on a different runtime thread. The task
can also be moved between threads after being spawned.

Tasks in Tokio are very lightweight. Under the hood, they require only a single
allocation and 64 bytes of memory. Applications should feel free to spawn
thousands, if not millions of tasks.

https://tokio.rs/tokio/tutorial/spawning

### Actors

Nothing in the actor model requires that each actor is its own thread. To the
contrary, most actor systems suggest that there should be a large number of
actors, and so each actor should map to a task rather than a thread. After all,
actors require exclusive access to their wrapped resources only when they
execute, and do not care whether they are on a thread of their own or not. In
fact, very frequently, the actor model is used in conjunction with the worker
pool model—for example, an application that uses the multi- threaded
asynchronous runtime Tokio can spawn an asynchronous task for each actor, and
Tokio will then make the execution of each actor a job in its worker pool. Thus,
the execution of a given actor may move from thread to thread in the worker pool
as the actor yields and resumes, but every time the actor executes it maintains
exclusive access to its wrapped resource.

(Excerpt from Rust for Rustaceans)

## Examples

Beginner's Guide to Concurrent Programming: Coding a Multithreaded Chat Server
using Tokio:

https://github.com/pretzelhammer/rust-blog/blob/master/posts/chat-server.md
