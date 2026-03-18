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

use std::sync::mpsc::{self};
use tokio::sync::oneshot;

/// An actor is split into two parts: the task and the handle.
///
/// The task is the independently spawned Tokio task that actually performs
/// the duties of the actor, and the handle is a struct that allows you to
/// communicate with the task.
struct MyActor {
    receiver: mpsc::Receiver<ActorMessage>,
    next_id: u32,
}

enum ActorMessage {
    GetUniqueId { respond_to: oneshot::Sender<u32> },
}

impl MyActor {
    fn new(receiver: mpsc::Receiver<ActorMessage>) -> Self {
        MyActor {
            receiver,
            next_id: 0,
        }
    }

    fn handle_message(&mut self, msg: ActorMessage) {
        match msg {
            ActorMessage::GetUniqueId { respond_to } => {
                self.next_id += 1;

                // The `let _ =` ignores any errors when sending.
                //
                // This can happen if the `select!` macro is used
                // to cancel waiting for the response.
                let _ = respond_to.send(self.next_id);
            }
        }
    }
}

async fn run_my_actor(mut actor: MyActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }
}

/// Now that we have the actor itself, we also need a handle to the actor.
///
/// A handle is an object that other pieces of code can use to talk to the actor,
/// and is also what keeps the actor alive.
#[derive(Clone)]
pub struct MyActorHandle {
    sender: mpsc::Sender<ActorMessage>,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
