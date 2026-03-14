---
title: Finite State Machine
---

A **state machine** (or finite state machine, FSM) is a behavioral model where a
system can exist in exactly one of a finite number of states at any given time,
and transitions between states are triggered by specific events or conditions.
Rust is particularly well-suited for state machines because its type system can
enforce valid transitions either at runtime (using enums) or at compile time
(using the typestate
pattern).[^1](https://www.itemis.com/en/products/itemis-create/documentation/user-guide/overview_what_are_state_machines)

---

## Core Concepts

Three building blocks define any state
machine:[^2](https://www.ni.com/docs/en-US/bundle/patools/page/what-is-a-state-machine.html)

- **States** – distinct situations a system can be in (e.g., `Created`, `Paid`,
  `Shipped`)
- **Transitions** – rules that move the system from one state to another
- **Events** – inputs/triggers that cause a transition (e.g., "payment
  received")

A system can only be in **one state at a time**, and transitions that don't
exist for the current state are simply
invalid.[^3](https://stately.ai/blog/2023-10-02-what-is-a-state-machine)

---

## Approach 1: Enum-Based State Machine

The simplest Rust approach uses `enum` variants to represent states and pattern
matching to handle transitions. Here's an order-processing
example:[^4](https://oneuptime.com/blog/post/2026-02-01-rust-state-machines/view)

```rust
#[derive(Debug, Clone)]
enum OrderState {
    Created { items: Vec<String>, total: f64 },
    Paid { items: Vec<String>, total: f64, payment_id: String },
    Shipped { items: Vec<String>, tracking_number: String },
    Delivered { delivered_at: String },
    Cancelled { reason: String },
}

#[derive(Debug)]
struct Order {
    id: String,
    state: OrderState,
}

impl Order {
    fn new(id: String, items: Vec<String>, total: f64) -> Self {
        Order { id, state: OrderState::Created { items, total } }
    }

    // Transition only valid from Created state
    fn pay(self, payment_id: String) -> Result<Self, &'static str> {
        match self.state {
            OrderState::Created { items, total } => Ok(Order {
                id: self.id,
                state: OrderState::Paid { items, total, payment_id },
            }),
            _ => Err("Can only pay for orders in Created state"),
        }
    }

    fn ship(self, tracking_number: String) -> Result<Self, &'static str> {
        match self.state {
            OrderState::Paid { items, .. } => Ok(Order {
                id: self.id,
                state: OrderState::Shipped { items, tracking_number },
            }),
            _ => Err("Can only ship paid orders"),
        }
    }
}

fn main() {
    let order = Order::new("ORD-001".to_string(), vec!["Widget".to_string()], 99.99);
    let order = order.pay("PAY-123".to_string()).unwrap();
    let order = order.ship("TRACK-456".to_string()).unwrap();
    println!("{:?}", order);
}
```

This catches invalid transitions **at runtime** via `Result`. It's simple and
easy to understand, but bugs only surface when that code path is
hit.[^4](https://oneuptime.com/blog/post/2026-02-01-rust-state-machines/view)

---

## Approach 2: Typestate Pattern (Compile-Time Safety)

The **typestate pattern** encodes state into the _type itself_ using generics,
so the Rust compiler rejects invalid transitions before your code ever
runs.[^4](https://oneuptime.com/blog/post/2026-02-01-rust-state-machines/view)

```rust
use std::marker::PhantomData;

// Zero-sized marker types — no runtime cost
struct Created;
struct Paid   { payment_id: String }
struct Shipped { tracking_number: String }

struct Order<St> {
    id: String,
    items: Vec<String>,
    total: f64,
    state_data: St,
}

// Methods only available on Order<Created>
impl Order<Created> {
    fn new(id: String, items: Vec<String>, total: f64) -> Self {
        Order { id, items, total, state_data: Created }
    }

    fn pay(self, payment_id: String) -> Order<Paid> {
        Order { id: self.id, items: self.items, total: self.total,
                state_data: Paid { payment_id } }
    }
}

// Methods only available on Order<Paid>
impl Order<Paid> {
    fn ship(self, tracking_number: String) -> Order<Shipped> {
        Order { id: self.id, items: self.items, total: self.total,
                state_data: Shipped { tracking_number } }
    }
}

fn main() {
    let order = Order::new("ORD-001".to_string(), vec!["Widget".to_string()], 49.99);
    let paid  = order.pay("PAY-123".to_string());   // ✅ Created → Paid
    let shipped = paid.ship("TRACK-456".to_string()); // ✅ Paid → Shipped

    // ❌ This would FAIL TO COMPILE — ship() doesn't exist on Order<Created>
    // let bad = order.ship("TRACK-789".to_string());
}
```

Because `ship()` is only implemented on `Order<Paid>`, calling it on an unpaid
order is a **compile-time error**, not a runtime
bug.[^4](https://oneuptime.com/blog/post/2026-02-01-rust-state-machines/view)

---

## Which Approach to Choose?

|                                   | Enum-Based                | Typestate Pattern                  |
| :-------------------------------- | :------------------------ | :--------------------------------- |
| **Error detection**               | Runtime                   | Compile time                       |
| **Complexity**                    | Low                       | Medium–High                        |
| **Dynamic state** (e.g., from DB) | ✅ Easy                   | ❌ Needs wrapper enum              |
| **IDE autocomplete**              | Shows all methods         | Shows only valid methods           |
| **Best for**                      | Prototyping, simple flows | Financial, safety-critical systems |

In practice, many production systems use **both**: typestate for core business
logic, with an enum wrapper (`AnyOrder`) for serialization and dynamic event
handling.[^4](https://oneuptime.com/blog/post/2026-02-01-rust-state-machines/view)

[^5]: https://www.freecodecamp.org/news/state-machines-basics-of-computer-science-d42855debc66/

[^6]: https://www.youtube.com/watch?v=sosnUnI-vco

[^7]: https://www.reddit.com/r/learnprogramming/comments/1g5yxci/state_machines_for_a_beginner/

[^8]: https://github.com/Lifestreams-ai/statemachine

[^9]: https://uk.mathworks.com/discovery/state-machine.html

[^10]: https://docs.rs/simple_statemachine

[^11]: https://www.reddit.com/r/rust/comments/18fugz0/state_machines_implementation/

[^12]: https://www.stateworks.com/technology/understanding-state-machines/

[^13]: https://en.wikipedia.org/wiki/Finite-state_machine

[^14]: https://users.rust-lang.org/t/on-state-machines/114910

[^15]: https://sparxsystems.com/resources/tutorials/uml2/state-diagram.html
