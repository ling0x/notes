---
title: Interface
---

In Rust, the concept of "programming to an interface/supertype" is achieved
using traits — Rust's equivalent of interfaces or abstract supertypes.

1. Define the "Supertype" (Trait)

```rust
trait Animal {
    fn make_sound(&self);
}
```

2. Concrete Implementations

```rust
struct Dog;
struct Cat;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
}
```

## Static Dispatch (resolved at compile time — faster)

```rust
fn interact(animal: &impl Animal) {
    animal.make_sound();
}

let animal = Dog;
interact(&animal); // Works for any Animal implementor
```

## Dynamic dispatch (resolved at runtime — more flexible)

```rust
fn interact(animal: &dyn Animal) {
    animal.make_sound();
}

let animal: &dyn Animal = &Dog; // Variable typed as the trait, not Dog
animal.make_sound();
```

## Assigning at Runtime

```rust
fn get_animal(sound_type: &str) -> Box<dyn Animal> {
    match sound_type {
        "dog" => Box::new(Dog),
        _     => Box::new(Cat),
    }
}

fn main() {
    let animal = get_animal("dog"); // We don't know the concrete type here
    animal.make_sound();            // All we care about is make_sound()
}
```
