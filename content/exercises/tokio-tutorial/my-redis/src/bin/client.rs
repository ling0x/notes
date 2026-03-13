use bytes::Bytes;
use mini_redis::client;
use tokio::sync::mpsc;

#[derive(Debug)]
enum Command {
    Get { key: String },
    Set { key: String, val: Bytes },
}

#[tokio::main]
async fn main() {
    // The mpsc channel supports sending many values from many producers to
    // a single consumer
    //
    // Create a new channel with a capacity of at most 32.
    // It returns two values: a sender and a receiver
    let (tx, mut rx) = mpsc::channel(32);

    // Sending from multiple tasks is done by cloning the Sender
    let tx2 = tx.clone();

    // Both messages are sent to the single Receiver handle.
    // It is not possible to clone the receiver of an mpsc channel.
    let t1 = tokio::spawn(async move {
        let cmd = Command::Get {
            key: "foo".to_string(),
        };
        tx.send(cmd).await.unwrap();
    });

    let t2 = tokio::spawn(async move {
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
        };
        tx2.send(cmd).await.unwrap();
    });

    // Spawn a task that processes messages from the channel.
    // First, a client connection is established to Redis.
    // Then, received commands are issued via the Redis connection.
    //
    // The `move` keyword is used to **move** ownership of `rx` into the task.
    let manager = tokio::spawn(async move {
        // Establish a connection to the server
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // Start receiving messages
        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key } => {
                    client.get(&key).await;
                }
                Set { key, val } => {
                    client.set(&key, val).await;
                }
            }
        }
    });

    // At the bottom of the main function, we .await the join handles to ensure
    // the commands fully complete before the process exits.
    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
