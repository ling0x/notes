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

use tokio::sync::{mpsc, oneshot};

/// An actor is split into two parts: the task and the handle.
///
/// The task is the independently spawned Tokio task that actually performs
/// the duties of the actor, and the handle is a struct that allows you to
/// communicate with the task.
struct MyActor {
    receiver: mpsc::Receiver<ActorMessage>,
    next_id: u32,
}

/// The ActorMessage enum defines the kind of messages we can send to the actor.
///
/// By using an enum, we can have many different message types, and each message
/// type can have its own set of arguments. We return a value to the sender by
/// using an oneshot channel, which is a message passing channel that allows
/// sending exactly one message.
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
        // We match on the enum inside a handle_message method on the actor struct,
        // but that isn't the only way to structure this. One could also match on
        // the enum in the run_my_actor function. Each branch in this match could
        // then call various methods such as get_unique_id on the actor object.
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
    // We can detect when the actor should shut down by looking at failures to
    // receive messages. In our example, this happens in the following while loop:
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }
}

/// Now that we have the actor itself, we also need a handle to the actor.
///
/// A handle is an object that other pieces of code can use to talk to the actor,
/// and is also what keeps the actor alive.
///
/// Derive Clone: Since the channel allows multiple producers, we can freely
/// clone our handle to the actor, allowing us to talk to it from multiple places.
#[derive(Clone)]
pub struct MyActorHandle {
    sender: mpsc::Sender<ActorMessage>,
}

impl MyActorHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = MyActor::new(receiver);
        tokio::spawn(run_my_actor(actor));

        Self { sender }
    }

    pub async fn get_unique_id(&self) -> u32 {
        let (send, recv) = oneshot::channel();
        let msg = ActorMessage::GetUniqueId { respond_to: send };

        // Ignore send errors. If this send fails, so does the
        // recv.await below. There's no reason to check for the
        // same failure twice.
        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor task has been killed")
    }
}

#[tokio::main]
async fn main() {
    // 1. Creating the handle also spawns the actor task automatically (inside MyActorHandle::new)
    let actor_handle = MyActorHandle::new();

    // 2. Send a message to the actor and await the response
    let id1 = actor_handle.get_unique_id().await;
    println!("Got id: {}", id1); // prints 1

    let id2 = actor_handle.get_unique_id().await;
    println!("Got id: {}", id2); // prints 2

    // 3. Clone the handle to show multiple owners can talk to the same actor
    let handle2 = actor_handle.clone();
    let id3 = handle2.get_unique_id().await;
    println!("Got id from cloned handle: {}", id3); // prints 3
}
