---
title: Data Streams
---

## Data Streams and Pipe

<img src="/whiteboards/data_stream.png" alt="Data Stream" width="100%">

## Passing Data Streams between rust binaries

We can pass data directly between binaries, they don't have to go through a
networking layers http server with ports, which adds unecessary overhead when it
is not needed.

<img src="/whiteboards/pipe_between_binaries.png" alt="Data Stream" width="100%">

## Making your own “filter” binary

If you want your Rust binary to be the thing that “processes stdout from another
binary”, just read from stdin and write to stdout:

```rust
use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    for line in stdin.lock().lines() {
        let line = line?;
        // transform line
        let processed = line.replace("foo", "bar");
        writeln!(out, "{processed}")?;
    }

    Ok(())
}
```

Then in a shell you can do `producer_cmd | your_rust_filter | consumer_cmd`.

## Use processed data as stdin to another binary

To “chain” binaries but keep Rust in the middle, you can run a second command
and write your processed data into its stdin.

```rust
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    // First binary: produces data
    let mut producer = Command::new("producer_cmd")
        .arg("some_arg")
        .stdout(Stdio::piped())
        .spawn()?;

    let producer_stdout = producer
        .stdout
        .take()
        .expect("failed to capture producer stdout");
    let reader = BufReader::new(producer_stdout);

    // Second binary: consumes processed data
    let mut consumer = Command::new("consumer_cmd")
        .arg("another_arg")
        .stdin(Stdio::piped())
        .spawn()?;

    let consumer_stdin = consumer
        .stdin
        .take()
        .expect("failed to open consumer stdin");

    // Process loop: read from producer, transform, send to consumer
    let mut writer = consumer_stdin;
    for line in reader.lines() {
        let line = line?;
        let processed = format!("prefix: {line}\n"); // any transformation
        writer.write_all(processed.as_bytes())?;
    }
    // Close consumer stdin so it can finish
    drop(writer);

    // Wait for both processes to exit
    let prod_status = producer.wait()?;
    let cons_status = consumer.wait()?;

    eprintln!("producer: {prod_status}, consumer: {cons_status}");

    Ok(())
}
```

Conceptually this is equivalent to
`producer_cmd | my_rust_filter | consumer_cmd`, but Rust is the middle filter,
so you never rely on shell piping.

## Async Pipes with Tokio

Use Tokio's async I/O to pipe binary data. `tokio::process::Command` mirrors the
sync API but uses `AsyncRead/AsyncWrite` traits, letting you `.await` on reads
and writes without blocking threads.

```rust
use tokio::process::Command;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Stdio;

#[tokio::main]
async fn main() {
    let mut child = Command::new("xxd")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn");

    let binary_data: Vec<u8> = (0u8..=255).collect(); // 256 bytes of raw binary

    // Write asynchronously
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(&binary_data).await.expect("Write failed");
        // Drop stdin to close pipe (send EOF)
    }

    // Read asynchronously
    let mut output = Vec::new();
    if let Some(mut stdout) = child.stdout.take() {
        stdout.read_to_end(&mut output).await.expect("Read failed");
    }

    child.wait().await.expect("Child failed");
    println!("Received {} bytes of output", output.len());
}
```

For streaming, long-running binaries, use tokio_process_tools which wraps
Tokio's process API with real-time line/byte inspection via channels. ​

## Fan-In: Many Binaries → One Receiver

This is a classic multi-producer, single-consumer (MPSC) pattern. You don't need
special OS-level listeners — Tokio's mpsc channel is purpose-built for this.

The architecture:

- Each producer binary is spawned as a child process with stdout(Stdio::piped())

- Each gets a clone of the mpsc::Sender

- A single async task owns the Receiver and processes all incoming data

```rust
use tokio::process::Command;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::sync::mpsc;
use std::process::Stdio;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<(usize, Vec<u8>)>(32);

    let producers = vec!["producer_a", "producer_b", "producer_c"];

    for (id, binary) in producers.iter().enumerate() {
        let tx = tx.clone(); // clone sender for each producer

        let mut child = Command::new(binary)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn producer");

        tokio::spawn(async move {
            let mut stdout = BufReader::new(child.stdout.take().unwrap());
            let mut buf = Vec::new();

            loop {
                let mut chunk = vec![0u8; 4096];
                match stdout.read(&mut chunk).await {
                    Ok(0) => break,           // EOF
                    Ok(n) => {
                        chunk.truncate(n);
                        tx.send((id, chunk)).await.unwrap();
                    }
                    Err(e) => { eprintln!("Error from producer {}: {}", id, e); break; }
                }
            }
            child.wait().await.unwrap();
        });
    }

    drop(tx); // drop original sender so rx knows when all producers are done

    // Single receiver handles all producers
    while let Some((producer_id, data)) = rx.recv().await {
        println!("Producer {}: {} bytes", producer_id, data.len());
        // process binary data here...
    }
}
```

Do You Need an Event Listener? Not in the traditional sense. Tokio's scheduler
is itself an event-driven async runtime — when no data is available, tasks are
suspended and CPU is yielded, not wasted in a spin loop.

# Delimiter

A delimiter is a character or group of characters that marks where one piece of
data ends and the next one begins.

In computing and data formats, common delimiters include commas, tabs, spaces,
semicolons, and pipes, such as in CSV files where commas separate values. In
general English dictionaries, it is defined as a character that marks the
beginning or end of a unit of data. In maths and programming, symbols like
parentheses (), quotes " " and braces {} also act as delimiters because they
enclose or bound expressions or code blocks.
