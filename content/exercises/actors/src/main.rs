//! An exercise to create a simple actor
//!
//! What is an actor?
//!
//! The basic idea behind an actor is to spawn a self-contained task that performs
//! some job independently of other parts of the program.
//!
//! Typically these actors communicate with the rest of the program through the
//! use of message passing channels.
//!
//! Since each actor runs independently, programs designed using them are
//! naturally parallel.
//!
//! Things to pay attention to:
//! 1. Where to put the tokio::spawn call.
//! 2. Struct with run method vs bare function.
//! 3. andles to the actor.
//! 4. Backpressure and bounded channels.
//! 5. Graceful shutdown.

use std::sync::mpsc;
use tokio::sync::oneshot;

struct MyActor {
    receiver: mpsc::Receiver<ActorMessage>,
    next_id: u32,
}

enum ActorMessage {
    GetUniqueId { respond_to: oneshot::Sender<u32> },
}

fn main() {
    println!("Hello, world!");
}
