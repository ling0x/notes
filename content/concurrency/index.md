---
title: Concurrency
---

Concurrent programming is the practice of performing multiple tasks during
overlapping periods of time. It’s not the same as parallelism; in concurrency,
different sequences of operations progress independently, though not necessarily
simultaneously. This concept is quite advanced in programming, as it often
involves managing [threads](/content/operating_systems/thread.md), locks, and
avoiding pitfalls like race conditions and
[deadlocks](https://tokio.rs/tokio/tutorial/shared-state#holding-a-mutexguard-across-an-await)—making
concurrent code challenging to implement correctly. By structuring programs as
independent processes that cooperate in a defined manner, developers can achieve
more flexible and efficient designs. While these designs aren’t always parallel,
the concurrency approach provides significant benefits in scalability and
responsiveness.

### Question: Whats the difference between concurrency and parallelism?

First, start to understand why we want to make a distinction between parallel
and concurrent in the first place!

> The why has everything to do with resource utilization and efficiency.

> Efficiency is the (often measurable) ability to avoid wasting materials,
> energy, effort, money, and time in doing something or in producing a desired
> result.

> Parallelism is increasing the resources we use to solve a task. It has nothing
> to do with efficiency.

> Concurrency has everything to do with efficiency and resource utilization.
> Concurrency can never make one single task go faster. It can only help us
> utilize our resources better and thereby finish a set of tasks faster.

(Asynchronous Programming in Rust)

### Question: Whats the difference between async programming and concurrent programming?

[Async programming](/concurrency/async_programming/index.md) is a way to
implement concurrency without relying on OS threads, while concurrent
programming is the broader idea of making a program do multiple things at once
or appear to do so. In the Rust async book, concurrent programming includes
thread-based models, but async programming keeps concurrency inside the program
and uses an async runtime plus explicit yielding with await instead of
OS-managed thread scheduling.

#### Core difference:

The Rust async book says concurrent programming means a program does multiple
things at the same time, or at least appears to. It also notes that threads are
one form of concurrent programming, where each thread is written sequentially
and the operating system runs those threads concurrently.

By contrast, async programming moves that coordination into your program rather
than the operating system. In Rust, an async runtime manages tasks, and tasks
give up control explicitly when they hit .await, which lets other tasks make
progress.

#### A simple way to think about it is this:

Concurrent programming asks, “How can multiple things make progress?”

Async programming answers, “We’ll do that inside one runtime, using lightweight
tasks and await.”

### Question: So does async programming avoid using threads?

Not exactly. Async programming does not require one OS thread per task, but it
can still use threads underneath. In Rust, the async model lets many lightweight
tasks run without creating extra threads for each task, while the runtime may
still use a small number of threads behind the scenes to drive those tasks.
[rust-book](https://rust-book.cs.brown.edu/ch17-02-concurrency-with-async.html),
[rust-lang](https://rust-lang.github.io/async-book/01_getting_started/02_why_async.html)

What async avoids What async mainly avoids is the “one task = one thread” model.
The Rust async material explains that you can run concurrent async work without
spawning extra operating-system threads for each small job, which is why async
can handle far more tasks with lower CPU and memory overhead.

What may still use threads An async runtime often does use threads internally.
The Rust book notes that, in practice, an async runtime might rely on
operating-system threads under the hood even though your async code is written
in terms of tasks and await, not manually managed threads.

#### Simple way to think about it A good shorthand is:

- Threads are one way to get concurrency. ​

- Async is another way to structure concurrency, usually with many tasks
  multiplexed onto a small number of threads.

So the precise answer is: async programming does not mean “no threads at all”;
it usually means “not one thread per unit of work.”
