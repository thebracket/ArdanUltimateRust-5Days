# Data Collection Server Version 0.1

Now that we've got a daemon that can collect data and send it, we need to build a server that can receive it. Let's start very simply---we'll build the bare minimum and print out what we receive.

> The initial server is in `code/05_server/server_v1`.

Create a new project, `server` with `cargo new`. We're not trying to be lightweight on the server, but we'll try and be somewhat efficient. We'll worry about the web and SQL parts later---for now, it's just a Tokio-based TCP server.

## Add Dependencies

You just need the two external dependencies for now: 
```bash
cargo add tokio -F full
cargo add anyhow
```

You'll also need a dependency on your shared library:

```toml
[dependencies]
tokio = { version = "1.28.2", features = ["full"] }
shared_v1 = { path = "../shared_v1" }
```

## Collecting Data

Create a new file in the `src` directory named `collector.rs`. Leave it empty, and fill `main.rs` with the boilerplate for an empty Tokio project, including `collector` as a module:

```rust
mod collector;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    Ok(())
}

```

## Listening for Connections

In the `collector` module, we'll start by creating a TCP listener. We'll use the `anyhow` crate to handle errors, and the `tokio::net::TcpListener` to create a listener. We'll also use the `tokio::net::TcpStream` to handle the connections.

```rust
pub async fn data_collector() -> anyhow::Result<()> {
    // Listen for TCP connections on the data collector address
    let listener = TcpListener::bind(DATA_COLLECTOR_ADDRESS).await?;

    // Loop forever, accepting connections
    loop {
        // Wait for a new connection
        let (socket, address) = listener.accept().await?;
        tokio::spawn(new_connection(socket, address));
    }
}
```

You can write all the logic inside the `spawn` call as a closure, but it's easier to read as a function. Rust is really good at inlining functions, so don't worry about function call overhead.

## Handling Connections

The `new_connection` function is quite simple, because we've already created the decoding logic:

```rust
async fn new_connection(mut socket: TcpStream, address: SocketAddr) {
    println!("New connection from {address:?}");
    let mut buf = vec![0u8; 1024];
    loop {
        let n = socket
            .read(&mut buf)
            .await
            .expect("failed to read data from socket");

        if n == 0 {
            println!("No data received - connection closed");
            return;
        }

        println!("Received {n} bytes");
        let received_data = decode_v1(&buf[0..n]);
        println!("Received data: {received_data:?}");
    }
}
```

So the flow here is:
    * A new connection arrives
        * A new async task is spawned just for that connection.
        * The connection task reads data from the socket.
        * The connection task tries to decode the data.
    * The primary collector keeps running, accepting new connections.

## Running the Server

Lastly, we need to make the `main` function spawn the collector task and wait for it to finish:

```rust
mod collector;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let handle = tokio::spawn(collector::data_collector());

    // Wait for the data collector to finish
    handle.await??; // Two question marks - we're unwrapping the task result, and the result from running the collector.
    Ok(())
}
```

Now run the server with `cargo run`. In a second window or panel, run the collector.

On the collector, you'll see a steady stream of `encoded 132 bytes`. On the server, you'll see something like this:

```
New connection from 127.0.0.1:65066
Received 131 bytes
Received data: (1686579624, SubmitData { collector_id: 0, total_memory: 34164006912, used_memory: 29945487360, average_cpu_usage: 8.190447 })
No data received - connection closed
```

Looking at Windows` "Resource Monitor", our first version is quite efficient:

* CPU usage is barely noticable.
* The Commit (KB) is 10,140 KB, or 10 MB. That's still surprisingly large, but it's not bad for a first version.

Now if you stop the server, the collector will crash. That's not really the behavior we want, but it's not bad for version 0.1!

Let's revisit the data collector and add some proper error handling.