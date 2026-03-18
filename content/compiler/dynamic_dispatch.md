---
title: Dynamic Dispatch
---

**Dynamic dispatch** is when you have multiple types that something might be,
but you don't know which one until runtime. So you **dynamically** figure out
which one of the methods to call on the type. Usually doing this requires the
use of **dyn** trait.

Dynamic dispatch is when the concrete type implementing a trait is resolved at
**runtime** rather than compile time. In Rust, this is done via trait objects
(`dyn Trait`), and the compiler implements it using a
[**vtable**](/compiler/vtable.md) — a table of function pointers generated for
each concrete type.

## [Actors](/concurrency/async_programming/actors.md) are dynamic dispatch

When you use actors, you don't need dynamic dispatch, because actors provide
dynamic dispatch on theri own.

Storing something in an actor is an alternative to `Box<dyn Trait>`.

```rust
struct MyActor<T: AsyncRead +  AsyncWrite> {
  receiver: mpsc::Receiver<ActorMessage>,
  connection: T,
}
```

Here, the generic T doesn't leak to the sender.

```rust
#[derive(Clone)]
pub struct MyActorHandle {
  sender: mpsc::Sender<ActorMessage>,
}
```

Here, T could be TcpStream or UnixStream depending on the connection. Remote
connection uses TcpStream, whereas local connection uses UnixStream, for
example.

## Static vs Dynamic Dispatch

In **static dispatch**, the compiler monomorphizes generic functions — it
generates a separate copy of the function for each concrete type used. Calls are
resolved at compile time and can be inlined.

```rust
fn area<T: Shape>(s: &T) -> f64 {
    s.area() // resolved at compile time
}
```

In **dynamic dispatch**, you use a trait object. The compiler doesn't know the
concrete type at compile time, so it emits a vtable lookup at each call site.

```rust
fn area(s: &dyn Shape) -> f64 {
    s.area() // resolved at runtime via vtable
}
```

## How the Compiler Implements It

A `dyn Trait` value is a **fat pointer** — two machine words:

- A **data pointer** to the value itself
- A **vtable pointer** to a static table of function pointers for the concrete
  type

The vtable is emitted by the compiler for each `(ConcreteType, Trait)` pair.
When you call a method on a `dyn Trait`, Rust loads the function pointer from
the vtable and calls it indirectly. This means no inlining and a small overhead,
but it enables **heterogeneous collections** and **type erasure**.

```
Box<dyn Shape>
├── data ptr ──► [ Circle { radius: 3.0 } ]
└── vtable ptr ─► [ drop, size, align, area, ... ]
```

## Object Safety

Not every trait can be used as `dyn Trait`. A trait must be **object-safe** for
this. The key rules are:

- Methods must not return `Self`
- Methods must not have generic type parameters
- The trait must not require `Sized`

The compiler enforces this — if a trait is not object-safe, using `dyn Trait` is
a compile error.

## When to Use Dynamic Dispatch

Prefer dynamic dispatch when:

- You need a heterogeneous collection (e.g. `Vec<Box<dyn Plugin>>`)
- You want to hide a concrete type behind an abstraction boundary (e.g. plugin
  systems, renderers, handlers)
- Binary size matters more than the last bit of performance (monomorphization
  bloat is real)

Prefer static dispatch when call-site performance and inlining are critical, or
when the set of concrete types is small and known ahead of time.

## Relationship to Ownership

Dynamic dispatch is orthogonal to both concurrency and memory safety. You can
wrap a trait object in any ownership primitive:

- `Box<dyn Trait>` — heap-allocated, single owner
- `Rc<dyn Trait>` — reference-counted, single-threaded
- `Arc<dyn Trait + Send + Sync>` — reference-counted, multi-threaded

The ownership wrapper controls lifetime and thread safety; the vtable only
controls how methods are dispatched.
