---
title: Segfault
---

A segmentation fault (segfault) is a runtime error that occurs when a program
tries to access a memory location it is not permitted to access — such as
reading or writing outside its allocated memory bounds. The operating system
catches this illegal access and terminates the offending process, sending a
SIGSEGV signal on Unix-like systems or raising a STATUS_ACCESS_VIOLATION
exception on Windows.

## How It Happens

Program memory is divided into segments — text (instructions), data (global
variables), stack (local variables), and heap (dynamically allocated memory). A
segfault occurs when a reference falls outside the segment where a variable
resides, or when a write is attempted to a read-only segment. The most common
causes are: ​

- Null pointer dereference — accessing memory at address 0x0

- Buffer overflow — reading/writing past the end of an array

- Dangling pointer — using a pointer to memory that has already been freed

- Stack overflow — infinite recursion exhausting the stack

## Rust and Segfaults

Rust's ownership and borrow checker system is specifically designed to eliminate
segfaults in safe code at compile time. According to the Rust team, a Rust
program can only segfault in two scenarios: you used unsafe code that violates
memory safety guarantees, or the Rust compiler itself has a bug. ​

### Triggering a segfault with unsafe Rust

The most direct way is dereferencing a raw null or invalid pointer inside an
unsafe block:

```rust
fn main() {
    // Dereference an invalid memory address — instant segfault
    unsafe { *(0x1 as *mut i32) = 1 };
}
```

This writes to memory address 0x1, which is not mapped, causing the OS to send
SIGSEGV.

### Stack overflow via infinite recursion

```rust
fn recurse() {
    recurse(); // infinite recursion → stack overflow → crash
}

fn main() {
    recurse();
}
```

Rust's runtime catches this and typically raises SIGABRT with a "thread has
overflowed its stack" message rather than a raw SIGSEGV, but it is the same
underlying mechanism.

### Writing to read-only memory via unsafe

```rust
fn main() {
    let x: &str = "hello"; // stored in read-only memory
    let ptr = x.as_ptr() as *mut u8;
    unsafe {
        *ptr = b'H'; // writing to read-only segment → segfault
    }
}
```

## Safe Rust vs. Unsafe Rust

| Scenario                 | Safe Rust                             | `unsafe` Rust                        |
| :----------------------- | :------------------------------------ | :----------------------------------- |
| Null pointer dereference | Impossible — `Option<T>` used instead | Possible with raw pointers           |
| Buffer overflow          | Panics with bounds check              | Possible with raw pointer arithmetic |
| Dangling pointer         | Prevented by borrow checker           | Possible                             |
| Stack overflow           | Handled gracefully (abort)            | Same behaviour                       |

The key takeaway is that safe Rust **prevents segfaults by design** — the borrow
checker enforces memory safety rules at compile time that languages like C/C++
leave to the programmer. When you do need low-level control, `unsafe` blocks opt
out of these guarantees and reintroduce the risk.
