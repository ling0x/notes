use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

/// Multiple different commands are multiplexed over a single channel.
#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

/// Provided by the requester and used by the manager task to send the command
/// response back to the requester
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    // The mpsc channel supports sending many values from many producers to
    // a single consumer
    //
    // Create a new channel with a capacity of at most 32.
    // It returns two values: a sender and a receiver
    //
    // tx and rx:
    // This naming comes from electronics and networking, where signal lines
    // are often labeled TX (transmit) and RX (receive), and Rust’s channel
    // examples follow the same tradition to indicate which end sends and which
    // end receives.
    let (tx, mut rx) = mpsc::channel(32);

    // Sending from multiple tasks is done by cloning the Sender
    let tx2 = tx.clone();

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
            match cmd {
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;

                    // Calling send on oneshot::Sender completes immediately
                    // and does not require an .await. This is because send on
                    // a oneshot channel will always fail or succeed immediately
                    // without any form of waiting.
                    let _ = resp.send(res);
                }
                Command::Set { key, val, resp } => {
                    let res = client.set(&key, val).await;

                    // Calling send on oneshot::Sender completes immediately
                    // and does not require an .await. This is because send on
                    // a oneshot channel will always fail or succeed immediately
                    // without any form of waiting.
                    let _ = resp.send(res);
                }
            }
        }
    });

    // Both messages are sent to the single Receiver handle.
    // It is not possible to clone the receiver of an mpsc channel.
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "foo".to_string(),
            resp: resp_tx,
        };

        // Send the Get request
        tx.send(cmd).await.unwrap();

        // Await the response
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx,
        };

        // Send the SET request
        tx2.send(cmd).await.unwrap();

        // Await the response
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    // At the bottom of the main function, we .await the join handles to ensure
    // the commands fully complete before the process exits.
    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
