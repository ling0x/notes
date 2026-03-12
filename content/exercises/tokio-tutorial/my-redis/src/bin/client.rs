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
    let (tx, mut rx) = mpsc::channel(32);

    // Sending from multiple tasks is done by cloning the Sender
    let tx2 = tx.clone();

    // Both messages are sent to the single Receiver handle.
    // It is not possible to clone the receiver of an mpsc channel.
    tokio::spawn(async move {
        tx.send("sending from first handle").await.unwrap();
    });

    tokio::spawn(async move {
        tx2.send("sending from second handle").await.unwrap();
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {message}");
    }
}
