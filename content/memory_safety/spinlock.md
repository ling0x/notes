---
title: Spinlock Mechanism
---

A spinlock works by repeatedly "spinning" — checking an atomic `locked` flag in
a tight loop until it successfully flips it from `false` to `true`. The key
operations are:[^1]

- **`lock()`** — atomically swaps `locked` from `false` to `true`. If `locked`
  is _already_ `true`, it keeps retrying (the "spin")
- **`unlock()`** — stores `false` back into `locked`, releasing it for another
  thread

```
THREAD 1                              THREAD 2
────────────────────────────────────────────────────────────

  ┌──────────────────────────┐
  │  swap locked             │
  │  false → true (acquire)  │
  └────────────┬─────────────┘
lock()         │
               ▼
  ┌──────────────────────────┐        ┌──────────────────────────┐
  │    access protected data │        │  swap locked             │
  └────────────┬─────────────┘        │  true → true (acquire)   │
               │                      └────────────┬─────────────┘
unlock()       │                                   │  (spinning...)
               ▼                      ┌────────────▼─────────────┐
  ┌──────────────────────────┐        │  swap locked             │
  │  store false in locked   │        │  true → true (acquire)   │
  │       (release)          │        └────────────┬─────────────┘
  └────────────┬─────────────┘                     │  (spinning...)
               │                                   │
               │  happens-before                   │
               └───────────────────────────────────▼
                                      ┌──────────────────────────┐
                                      │  swap locked             │
                                      │  false → true (acquire)  │  ← lock() succeeds
                                      └────────────┬─────────────┘
                                    lock()         │
                                                   ▼
                                      ┌──────────────────────────┐
                                      │    access protected data │
                                      └────────────┬─────────────┘
                                    unlock()       │
                                                   ▼
                                      ┌──────────────────────────┐
                                      │  store false in locked   │
                                      │       (release)          │
                                      └──────────────────────────┘
```

- Thread 1 (left column) acquires the lock immediately (swaps false → true),
  accesses the protected data, then releases by storing false. ​

- Thread 2 (right column) spins repeatedly — each swap returns true → true,
  meaning the lock is still held — until Thread 1's unlock() stores false. ​

- The happens-before arrow (the pink curved arrow in the original) is shown as
  the └──────────────────────────────────────────────────────▶ line connecting
  Thread 1's release to Thread 2's successful acquire. This is the critical
  guarantee that prevents concurrent data access.

## Why the Second Thread is Blocked

Looking at the diagram, Thread 2 (right column) attempts its own `lock()` call
but keeps getting `true → true` swaps, meaning it sees the lock is still held.
It can only proceed when Thread 1's `unlock()` stores `false` — at that point,
Thread 2's next swap finally succeeds (`false → true`), and _only then_ does it
access the protected data.[^2]

This is the **happens-before relationship** mentioned in the text: Thread 1's
`unlock()` (release) _happens before_ Thread 2's successful `lock()` (acquire),
which guarantees that any memory writes Thread 1 made to the shared data are
visible to Thread 2.[^3]

## The Concrete Guarantee

| Moment | Thread 1                       | Thread 2                     |
| :----- | :----------------------------- | :--------------------------- |
| T1     | Acquires lock (`false→true`)   | —                            |
| T2     | Accessing protected data       | Spinning (sees `true→true`)  |
| T3     | Releases lock (stores `false`) | Still spinning               |
| T4     | Done                           | Acquires lock (`false→true`) |
| T5     | —                              | Accessing protected data     |

The data access blocks (pink/maroon boxes in the diagram) are never active at
the same time — Thread 2 is stuck spinning until Thread 1 is completely done and
has unlocked. That's what "can't access the data concurrently" means: the
spinlock enforces **serial, not parallel** access to that critical section.[^1]
<span style="display:none">[^10][^11][^4][^5][^6][^7][^8][^9]</span>

<div align="center">⁂</div>

[^1]: https://www.shadecoder.com/topics/a-spinlock-a-comprehensive-guide-for-2025

[^2]: https://wiki.osdev.org/Spinlock

[^3]: https://en.wikipedia.org/wiki/Lock_(computer_science)

[^4]: image.jpg

[^5]: https://www.productteacher.com/quick-product-tips/understanding-mutual-exclusion-mutex

[^6]: https://stackoverflow.com/questions/53919851/how-spinlock-prevents-the-process-to-be-interrupted

[^7]: https://forums.swift.org/t/pitch-synchronous-mutual-exclusion-lock/69889

[^8]: https://student.cs.uwaterloo.ca/~cs350/F19/notes/synchronization-2up.pdf

[^9]: https://student.cs.uwaterloo.ca/~cs350/F20/notes/synchronization-2up.pdf

[^10]: https://de.wikipedia.org/wiki/Spinlock

[^11]: https://spcl.inf.ethz.ch/Teaching/2020-pp/lectures/PP-l17-BeyondLocks.pdf
