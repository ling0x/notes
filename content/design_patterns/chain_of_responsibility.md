---
title: Chain of Responsibility
---

<img src="/assets/mooncake.gif" alt="Chain" width="100%">

The Chain of Responsibility (CoR) pattern is a behavioral design pattern where a
request is passed along a chain of handlers — each handler either processes the
request or forwards it to the next one in the chain.

The three key components are: ​

- Handler — a trait defining the interface for handling requests and holding the
  successor link

- Successor — the next handler in the chain, stored as Option<Box<dyn Handler>>

- Request — the data passed along the chain

> The Rust community's guiding principle here is: static where you can, dynamic
> where you must.

### Example 1: Purchase Approval Chain (Dynamic Disptach)

The key pattern here is the Option<Box<dyn Approver>> field — it lets each
handler optionally own its successor and forward the request via
next.process_request(request).

```rust
struct PurchaseRequest {
    amount: f64,
}

trait Approver {
    fn set_successor(&mut self, successor: Box<dyn Approver>);
    fn process_request(&self, request: &PurchaseRequest);
}

struct Manager {
    successor: Option<Box<dyn Approver>>,
}

impl Approver for Manager {
    fn set_successor(&mut self, successor: Box<dyn Approver>) {
        self.successor = Some(successor);
    }
    fn process_request(&self, request: &PurchaseRequest) {
        if request.amount <= 1000.0 {
            println!("Manager approves ${}", request.amount);
        } else if let Some(ref next) = self.successor {
            next.process_request(request); // pass it up
        } else {
            println!("Cannot be approved.");
        }
    }
}

struct Director { successor: Option<Box<dyn Approver>> }

impl Approver for Director {
    fn set_successor(&mut self, successor: Box<dyn Approver>) {
        self.successor = Some(successor);
    }
    fn process_request(&self, request: &PurchaseRequest) {
        if request.amount <= 5000.0 {
            println!("Director approves ${}", request.amount);
        } else if let Some(ref next) = self.successor {
            next.process_request(request);
        } else {
            println!("Cannot be approved.");
        }
    }
}

struct President;

impl Approver for President {
    fn set_successor(&mut self, _: Box<dyn Approver>) {} // terminal node
    fn process_request(&self, request: &PurchaseRequest) {
        if request.amount <= 10000.0 {
            println!("President approves ${}", request.amount);
        } else {
            println!("Request denied.");
        }
    }
}

fn main() {
    let president = President;
    let mut director = Director { successor: Some(Box::new(president)) };
    let mut manager = Manager { successor: Some(Box::new(director)) };

    manager.process_request(&PurchaseRequest { amount: 500.0 });   // Manager approves
    manager.process_request(&PurchaseRequest { amount: 3000.0 });  // Director approves
    manager.process_request(&PurchaseRequest { amount: 8000.0 });  // President approves
    manager.process_request(&PurchaseRequest { amount: 15000.0 }); // Denied
}
```

### Example 2: Purchase Approval Chain (Generics - Zero-cost static dispatch)

This is verbose but gives you zero-cost static dispatch — no heap allocation, no
vtable lookup, all method calls resolved at compile time.

`Handler<Manager<Director<President>>>`

```rust
trait Handler {
    fn handle(&self, amount: f64);
}

// Terminal handler — no successor
struct President;

impl Handler for President {
    fn handle(&self, amount: f64) {
        if amount <= 10_000.0 {
            println!("President approves ${amount}");
        } else {
            println!("Request denied.");
        }
    }
}

// Generic handler wrapping the next handler N
struct Manager<N: Handler> {
    next: N,
}

impl<N: Handler> Handler for Manager<N> {
    fn handle(&self, amount: f64) {
        if amount <= 1_000.0 {
            println!("Manager approves ${amount}");
        } else {
            self.next.handle(amount); // static dispatch!
        }
    }
}

struct Director<N: Handler> {
    next: N,
}

impl<N: Handler> Handler for Director<N> {
    fn handle(&self, amount: f64) {
        if amount <= 5_000.0 {
            println!("Director approves ${amount}");
        } else {
            self.next.handle(amount);
        }
    }
}

fn main() {
    // The full type is Manager<Director<President>>
    let chain = Manager {
        next: Director {
            next: President,
        },
    };

    chain.handle(500.0);    // Manager approves
    chain.handle(3_000.0);  // Director approves
    chain.handle(8_000.0);  // President approves
    chain.handle(15_000.0); // Request denied
}
```

The compiler monomorphizes this — it generates a unique, optimized function for
each concrete type combination. No heap, no vtable.

#### Using impl Trait to Hide the Chain Type

If you want to hide the ugly Manager<Director<President>> type from callers, use
impl Trait as a return type:

```rust
fn build_chain() -> impl Handler {
    Manager {
        next: Director {
            next: President,
        },
    }
}

fn main() {
    let chain = build_chain(); // type is opaque to the caller
    chain.handle(3_000.0);
}
```

This still uses static dispatch under the hood — the compiler knows the exact
type, the caller just doesn't need to spell it out.

### Example 3: Enum-Based Variant (More Idiomatic)

Rust's enums offer a cleaner alternative when the set of handlers is fixed at
compile time:

```rust
enum SupportLevel {
    Basic,
    Intermediate,
    Critical,
}

fn handle_request(level: SupportLevel) {
    match level {
        SupportLevel::Basic       => println!("L1 Support handled it"),
        SupportLevel::Intermediate => println!("L2 Support handled it"),
        SupportLevel::Critical    => println!("L3 Support handled it"),
    }
}
```

This approach avoids heap allocation and dynamic dispatch entirely, but loses
the runtime flexibility of adding or reordering handlers.
