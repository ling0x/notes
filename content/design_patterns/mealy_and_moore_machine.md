---
title: Mealy and Moore Machine
---

# Compare Mealy and Moore state machines with Rust code

The key difference between Mealy and Moore
[state machines](/design_patterns/finite_state_machine.md) comes down to **where
the output lives**: in Moore machines, output is tied to the _state_; in Mealy
machines, output is tied to the _transition_ (state + input together).[^1]

---

## Moore Machine

In a **Moore machine**, each state has a fixed output associated with it,
regardless of how you arrived there or what the current input is. The output
only changes when you _enter a new state_ — making it synchronous and immune to
input glitches.[^2][^3]

Here's a traffic light modelled as a Moore machine — the output (light colour)
depends purely on the current state:

```rust
#[derive(Debug, Clone, PartialEq)]
enum TrafficState {
    Red,
    Green,
    Yellow,
}

#[derive(Debug)]
enum Input {
    Timer,
}

struct MooreMachine {
    state: TrafficState,
}

impl MooreMachine {
    fn new() -> Self {
        MooreMachine { state: TrafficState::Red }
    }

    // Output is derived from STATE alone — no input needed
    fn output(&self) -> &str {
        match self.state {
            TrafficState::Red    => "STOP",
            TrafficState::Green  => "GO",
            TrafficState::Yellow => "SLOW DOWN",
        }
    }

    // Transition: input triggers state change, output follows state
    fn transition(&mut self, input: Input) {
        self.state = match (&self.state, input) {
            (TrafficState::Red,    Input::Timer) => TrafficState::Green,
            (TrafficState::Green,  Input::Timer) => TrafficState::Yellow,
            (TrafficState::Yellow, Input::Timer) => TrafficState::Red,
        };
        // Output is read AFTER the state changes
        println!("State: {:?} → Output: {}", self.state, self.output());
    }
}

fn main() {
    let mut fsm = MooreMachine::new();
    println!("Initial output: {}", fsm.output()); // "STOP"
    fsm.transition(Input::Timer); // Green → "GO"
    fsm.transition(Input::Timer); // Yellow → "SLOW DOWN"
    fsm.transition(Input::Timer); // Red → "STOP"
}
```

Output is produced **after** entering the new state — `output()` takes `&self`
with no input parameter.[^3]

---

## Mealy Machine

In a **Mealy machine**, output is produced **on the transition** — it depends on
both the current state _and_ the input that triggered the move. This means
outputs can react instantly to inputs, and you typically need fewer states than
an equivalent Moore machine.[^4][^1]

Here's a coin-operated vending machine where the output depends on both state
and the coin inserted:

```rust
#[derive(Debug, Clone, PartialEq)]
enum VendingState {
    Idle,
    Has10p,
    Has20p,
}

#[derive(Debug)]
enum Coin {
    P10,
    P20,
}

// Output is produced ON the transition, not from the state alone
#[derive(Debug)]
enum Output {
    None,
    Dispense(&'static str),
    ReturnChange(u32),
}

struct MealyMachine {
    state: VendingState,
}

impl MealyMachine {
    fn new() -> Self {
        MealyMachine { state: VendingState::Idle }
    }

    // Returns (new_state, output) — output depends on BOTH state AND input
    fn transition(&mut self, coin: Coin) -> Output {
        let (next_state, output) = match (&self.state, coin) {
            (VendingState::Idle,  Coin::P10) => (VendingState::Has10p, Output::None),
            (VendingState::Idle,  Coin::P20) => (VendingState::Has20p, Output::None),
            (VendingState::Has10p, Coin::P10) => (VendingState::Has20p, Output::None),
            (VendingState::Has10p, Coin::P20) => (VendingState::Idle,   Output::Dispense("Chewing gum (30p)")),
            (VendingState::Has20p, Coin::P10) => (VendingState::Idle,   Output::Dispense("Chocolate (30p)")),
            (VendingState::Has20p, Coin::P20) => (VendingState::Idle,   Output::ReturnChange(10)),
        };
        self.state = next_state;
        output
    }
}

fn main() {
    let mut fsm = MealyMachine::new();
    println!("{:?}", fsm.transition(Coin::P10)); // None
    println!("{:?}", fsm.transition(Coin::P20)); // Dispense("Chewing gum (30p)")
    println!("{:?}", fsm.transition(Coin::P20)); // None
    println!("{:?}", fsm.transition(Coin::P10)); // Dispense("Chocolate (30p)")
}
```

Notice `transition()` takes **both** `&self` and `coin` to compute the output —
a dead giveaway of the Mealy model.[^5]

---

## Side-by-Side Comparison

| Feature                      | Moore                                    | Mealy                                       |
| :--------------------------- | :--------------------------------------- | :------------------------------------------ |
| **Output depends on**        | Current state only                       | Current state + input                       |
| **Output location**          | Attached to state                        | Attached to transition                      |
| **Rust signature**           | `fn output(&self)`                       | `fn transition(&mut self, input) -> Output` |
| **Number of states**         | More (one per output combo)              | Fewer [^4]                                  |
| **Output timing**            | After state change (synchronous)         | Immediately on input (asynchronous) [^2]    |
| **Input glitch sensitivity** | Immune                                   | More sensitive [^2]                         |
| **Best for**                 | Display/status outputs, digital circuits | Reactive systems, protocol parsers          |

---

## Key Takeaway

Both models are equally expressive — any Mealy machine can be converted to a
Moore machine by splitting states, and vice versa. In Rust, the distinction maps
cleanly: Moore output lives in a method that reads `&self` alone, while Mealy
output is returned from the transition method that takes both `&self` and the
input event.[^6][^5]
<span style="display:none">[^10][^11][^12][^13][^14][^15][^7][^8][^9]</span>

<div align="center">⁂</div>

[^1]: https://www.geeksforgeeks.org/theory-of-computation/difference-between-mealy-machine-and-moore-machine/

[^2]: https://www.youtube.com/watch?v=YiQxeuB56i0

[^3]: https://stackoverflow.com/questions/4009283/mealy-v-s-moore

[^4]: https://mil.ufl.edu/3701/classes/joel/16 Lecture.pdf

[^5]: https://comp.lang.forth.narkive.com/zJDmPu3N/mealy-vs-moore-fsm

[^6]: https://www.reddit.com/r/explainlikeimfive/comments/30uq6e/eli5_the_difference_between_a_mealey_machine_and/

[^7]: https://www.youtube.com/watch?v=kb-Ww8HaHuE

[^8]: https://forum.allaboutcircuits.com/threads/when-to-use-mealy-machine-and-when-to-use-moore-machine.190031/

[^9]: https://github.com/rust-cy/generic-state-machine-rs

[^10]: https://docs.rs/rust-fsm/

[^11]: https://users.rust-lang.org/t/how-to-create-complex-state-machines/82714

[^12]: https://www.reddit.com/r/FPGA/comments/vmlb5z/is_there_any_difference_in_my_implementation_of/

[^13]: https://oneuptime.com/blog/post/2026-02-01-rust-state-machines/view

[^14]: https://lib.rs/crates/edfsm

[^15]: https://blog.devgenius.io/building-robust-distributed-state-machines-in-rust-a-comprehensive-guide-ad1a358134df
