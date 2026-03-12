---
title: Stack and Heap
---

# Stack

## Call Stack

A region of memory the CPU uses automatically to track function calls, memory
pointers, local variables, and return addresses

## Rust-specific rule

[Borrowing Rules](../memory_safety/borrowing_rules.md)

# Heap

The heap is a region of a computer's memory used for dynamic memory allocation —
memory that is reserved and released at runtime rather than at compile time. It
is one of the two main memory areas programs use (alongside the stack), and it
gives programmers flexible control over how much memory to use and for how long.

## Heap vs. Stack

| Feature           | Heap                         | Stack                 |
| :---------------- | :--------------------------- | :-------------------- |
| Allocation timing | Runtime (dynamic)            | Compile time (static) |
| Size              | Large, flexible              | Small, fixed          |
| Management        | Manual or GC                 | Automatic (LIFO)      |
| Access scope      | Global (anywhere in program) | Local to function     |
| Speed             | Slightly slower              | Faster                |
| Risk              | Memory leaks, fragmentation  | Stack overflow        |

The stack follows a strict last-in, first-out (LIFO) structure, making it fast
but limited. The heap is more flexible but requires careful management — failure
to deallocate memory causes **memory leaks**, where memory becomes permanently
unavailable.
