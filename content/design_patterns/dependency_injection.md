---
title: Dependency Injection
---

**Dependency injection** (DI) is a design pattern where a component receives
its dependencies from an external source rather than creating them
itself.[^1](https://dev.to/sgchris/how-traits-enable-dependency-injection-in-rust-5a50)
This decouples components, making code easier to test, swap, and maintain.
In Rust, DI is achieved idiomatically through **traits** and **generics**
— without needing a framework — leveraging the compiler's type system to
enforce correctness at compile
time.[^2](https://jmmv.dev/2022/04/rust-traits-and-dependency-injection.html)

---

## Core Tool: Traits

In Rust, **traits** act as the contract (interface) that dependencies must
fulfill.[^1](https://dev.to/sgchris/how-traits-enable-dependency-injection-in-rust-5a50)
Define the behaviour, not the implementation:

```rust
pub trait Logger {
    fn log(&self, message: &str);
}

pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("[LOG]: {}", message);
    }
}
```

---

## Approach 1: Generics (Static Dispatch)

The preferred Rust approach — the concrete type is resolved at **compile
time**, producing zero-cost
abstractions.[^1](https://dev.to/sgchris/how-traits-enable-dependency-injection-in-rust-5a50)

```rust
pub struct Application<L: Logger> {
    logger: L,
}

impl<L: Logger> Application<L> {
    pub fn new(logger: L) -> Self {
        Self { logger }
    }

    pub fn run(&self) {
        self.logger.log("Application is running!");
    }
}

fn main() {
    let app = Application::new(ConsoleLogger);
    app.run(); // prints: [LOG]: Application is running!
}
```

`Application` only requires that `L` implements `Logger` — swap in any
logger without touching `Application`
itself.[^3](https://users.rust-lang.org/t/how-do-you-implement-dependency-injection-in-rust/213)

---

## Approach 2: Trait Objects (Dynamic Dispatch)

When you need runtime flexibility or to store mixed implementations in a
collection, use `Box<dyn
Trait>`.[^3](https://users.rust-lang.org/t/how-do-you-implement-dependency-injection-in-rust/213)

```rust
pub trait MessageSender {
    fn send(&self, msg: &str);
}

pub struct NotificationService {
    sender: Box<dyn MessageSender>,
}

impl NotificationService {
    pub fn new(sender: Box<dyn MessageSender>) -> Self {
        Self { sender }
    }

    pub fn notify(&self, msg: &str) {
        self.sender.send(msg);
    }
}
```

This introduces a small **runtime overhead** via vtable lookup, so prefer
generics unless dynamic dispatch is genuinely
needed.[^1](https://dev.to/sgchris/how-traits-enable-dependency-injection-in-rust-5a50)

---

## Approach 3: Enums (Closed Set)

If you have a fixed, known set of implementations, an enum avoids both
generics complexity and boxing
overhead.[^3](https://users.rust-lang.org/t/how-do-you-implement-dependency-injection-in-rust/213)

```rust
pub enum LoggerKind {
    Console,
    File(String),
}

impl Logger for LoggerKind {
    fn log(&self, message: &str) {
        match self {
            LoggerKind::Console => println!("[Console]: {}", message),
            LoggerKind::File(path) => println!("[File({})] {}", path, message),
        }
    }
}
```

---

## Why DI Shines in Testing

The real payoff is **mockability** — swap the real implementation with a
mock during
tests.[^1](https://dev.to/sgchris/how-traits-enable-dependency-injection-in-rust-5a50)

```rust
pub struct MockLogger {
    pub messages: std::cell::RefCell<Vec<String>>,
}

impl Logger for MockLogger {
    fn log(&self, message: &str) {
        self.messages.borrow_mut().push(message.to_string());
    }
}

#[test]
fn test_app_logs_on_run() {
    let mock = MockLogger { messages: Default::default() };
    let app = Application::new(&mock);
    app.run();
    assert!(mock.messages.borrow().contains(&"Application is running!".to_string()));
}
```

No real I/O, no external systems — pure, fast unit
tests.[^1](https://dev.to/sgchris/how-traits-enable-dependency-injection-in-rust-5a50)

---

## Choosing the Right Approach

|                                        | Generics              | `Box<dyn Trait>`        | Enum                   |
| :------------------------------------- | :-------------------- | :---------------------- | :--------------------- |
| **Dispatch**                           | Compile time (static) | Runtime (dynamic)       | Compile time (static)  |
| **Overhead**                           | Zero                  | Vtable lookup           | Zero                   |
| **Implementations**                    | Any (open set)        | Any (open set)          | Fixed (closed set)     |
| **Heterogeneous collections**          | ❌                    | ✅                      | ✅                     |
| **Best for**                           | Most cases            | Plugin-like flexibility | Known, finite variants |

For complex dependency graphs, consider a DI container crate such as
[rustyinject](https://github.com/AlexSherbinin/rustyinject).[^4](https://github.com/AlexSherbinin/rustyinject)

---

## Key Pitfall: Visibility Creep

Any type referenced in a **public** trait's function signature must also be
public.[^2](https://jmmv.dev/2022/04/rust-traits-and-dependency-injection.html)
If your trait is public but references an internal struct, you'll be forced
to expose that struct too — keep internal traits `pub(crate)` where
possible.[^2](https://jmmv.dev/2022/04/rust-traits-and-dependency-injection.html)
