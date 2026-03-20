---
title: Unix Socket
---

A Unix domain socket (UDS) is a special file on your filesystem (e.g.
`/tmp/app.sock`) that acts as a communication endpoint between processes on the
same machine. Unlike TCP sockets, data never leaves the OS kernel — it's copied
directly between process buffers — making it significantly faster for local IPC.
The server binds to the socket path, and one or more clients connect to it.

Key properties:

- Lives at a filesystem path, controlled by file permissions ​

- Supports stream mode (like TCP — ordered, reliable) and datagram mode (like
  UDP — message-based)

- Commonly used between e.g. Nginx ↔ PHP-FPM, apps ↔ local databases, or
  microservices on the same host

## Rust Examples: Multiple Producers → Single Receiver

The pattern here is: **multiple client threads each connect to the same Unix
socket**, and a single server receives all their messages. We'll use tokio for
async I/O.
[Unix Listener](https://docs.rs/tokio/latest/tokio/net/struct.UnixListener.html),
[Unix Stream](https://docs.rs/tokio/latest/tokio/net/struct.UnixStream.html)

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

### Example 1 — Basic Server (Single Receiver)

The server binds to a socket path and handles each incoming connection, printing
whatever data it receives.
[docs](https://docs.rs/tokio/latest/tokio/net/struct.UnixListener.html)

```rust
// server.rs
use tokio::net::UnixListener;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    let socket_path = "/tmp/mpsr.sock";
    let _ = std::fs::remove_file(socket_path); // clean up stale socket

    let listener = UnixListener::bind(socket_path).unwrap();
    println!("Server listening on {}", socket_path);

    loop {
        match listener.accept().await {
            Ok((mut stream, _addr)) => {
                // Spawn a task per connection — all funnel into this single server
                tokio::spawn(async move {
                    let mut buf = vec![0u8; 1024];
                    match stream.read(&mut buf).await {
                        Ok(n) if n > 0 => {
                            let msg = String::from_utf8_lossy(&buf[..n]);
                            println!("[Server] Received: {}", msg);
                        }
                        _ => eprintln!("[Server] Connection closed or error"),
                    }
                });
            }
            Err(e) => eprintln!("Accept error: {}", e),
        }
    }
}
```

### Example 2 — Multiple Producers (Clients)

Each producer runs in its own thread, connects to the socket, and sends a
message.

```rust
// client.rs
use tokio::net::UnixStream;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let socket_path = "/tmp/mpsr.sock";
    let num_producers = 5;

    let mut handles = vec![];

    for i in 0..num_producers {
        let path = socket_path.to_string();
        let handle = tokio::spawn(async move {
            match UnixStream::connect(&path).await {
                Ok(mut stream) => {
                    let msg = format!("Hello from producer {}", i);
                    stream.write_all(msg.as_bytes()).await.unwrap();
                    println!("[Producer {}] Sent: {}", i, msg);
                }
                Err(e) => eprintln!("[Producer {}] Failed to connect: {}", i, e),
            }
        });
        handles.push(handle);
    }

    for h in handles {
        h.await.unwrap();
    }
}
```

### Example 3 — Combined: Server + Producers in One Binary

A self-contained example where the server is spawned as a background task, and 5
producers send concurrently.

```rust
use tokio::net::{UnixListener, UnixStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let socket_path = "/tmp/mpsr_combined.sock";
    let _ = std::fs::remove_file(socket_path);

    // --- Spawn the single receiver (server) ---
    let server_path = socket_path.to_string();
    tokio::spawn(async move {
        let listener = UnixListener::bind(&server_path).unwrap();
        println!("[Server] Listening...");
        loop {
            if let Ok((mut stream, _)) = listener.accept().await {
                tokio::spawn(async move {
                    let mut buf = vec![0u8; 256];
                    if let Ok(n) = stream.read(&mut buf).await {
                        println!("[Server] Got: {}", String::from_utf8_lossy(&buf[..n]));
                    }
                });
            }
        }
    });

    // Give the server a moment to bind
    sleep(Duration::from_millis(50)).await;

    // --- Spawn multiple producers ---
    let mut handles = vec![];
    for i in 0..5 {
        let path = socket_path.to_string();
        handles.push(tokio::spawn(async move {
            let mut stream = UnixStream::connect(&path).await.unwrap();
            let msg = format!("Message from producer {}", i);
            stream.write_all(msg.as_bytes()).await.unwrap();
            println!("[Producer {}] Sent.", i);
        }));
    }

    for h in handles {
        h.await.unwrap();
    }

    sleep(Duration::from_millis(100)).await; // let server print all messages
}
```

Expected output (order may vary since tasks run concurrently):

```text
[Server] Listening...
[Producer 0] Sent.
[Producer 2] Sent.
[Server] Got: Message from producer 0
[Server] Got: Message from producer 2
...
```

### How the MPSR Pattern Works Here

Each producer independently connects and sends — the Unix socket listener
naturally queues incoming connections. The tokio::spawn per connection is what
makes this a multiple-producer, single-receiver funnel: one server loop accepts
all connections, but each connection is handled concurrently. If you also want
to aggregate messages into a single channel, you can pair this with
std::sync::mpsc or tokio::sync::mpsc inside the server's accept loop.
[mpsc](https://blog.softwaremill.com/multithreading-in-rust-with-mpsc-multi-producer-single-consumer-channels-db0fc91ae3fa)
