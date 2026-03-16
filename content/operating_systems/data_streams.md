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
