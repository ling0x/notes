---
title: Operating Systems
---

## OS-level view

A [mutex (mutual exclusion lock)](/memory_safety/mutex.md) is a synchronization
primitive that ensures only one thread at a time enters a critical section that
uses some shared data.

If a thread tries to lock a mutex that is already locked, the OS (or runtime)
either makes it spin briefly or puts it to sleep and queues it until the mutex
is unlocked.

The OS uses low-level tools (like Linux futex on an integer in memory) plus
atomics and memory fences to implement this reliably across cores.

- [System Call(syscall)](/operating_systems/system_call.md)
- [User Space and Kernel Space](/operating_systems/user_space_and_kernel_space.md)
- [Thread](/operating_systems/thread.md)
