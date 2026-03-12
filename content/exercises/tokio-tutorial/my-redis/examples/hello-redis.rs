use mini_redis::{Result, client};

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address
    //
    // connect function asynchronously establishes a TCP connection with the
    // specified remote address. Once the connection is established, a client
    // handle is returned. Even though the operation is performed asynchronously,
    // the code we write looks synchronous. The only indication that the operation
    // is asynchronous is the .await operator.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    print!("got value from the server; result={result:?}");

    Ok(())
}
