# Error Handling in the Collection Daemon

It's obviously not desirable to have the collection daemon crash if the server isn't available. We need to handle errors in a way that allows us to recover from them.

> The code for this is in `code/05_server/collector_v2`.

## Handling Errors

Open up your data collector again, and add `thiserror` as a dependency with `cargo add thiserror`.

Create a new file in `src`, named `errors.rs` and add `mod errors;` to the top of `main.rs` to use it. We're going to define an error type in here:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CollectorError {
    #[error("Unable to connect to the server")]
    UnableToConnect,
}
```

Now in `sender.rs`, let's change the function to emit `CollectionError` errors if a failure occurs:

```rust
use crate::errors::CollectorError;
use shared_v1::{CollectorCommandV1, DATA_COLLECTOR_ADDRESS};
use std::io::Write;

pub fn send_command(command: CollectorCommandV1) -> Result<(), CollectorError> {
    let bytes = shared_v1::encode_v1(command);
    println!("Encoded {} bytes", bytes.len());
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS)
        .map_err(|_| CollectorError::UnableToConnect)?;
    stream
        .write_all(&bytes)
        .map_err(|_| CollectorError::UnableToSendData)?;

    Ok(())
}
```

We're using `map_err` to translate the internal errors into our own error format.

Now in `main.rs`, let's simply ignore the error for now:

```rust
// Listen for commands to send
while let Ok(command) = rx.recv() {
    let _ = sender::send_command(command);
}
```

> The data collector doesn't create any errors to unwrap, so we can leave it alone.

Now you can run the collector without the server, and it will silently drop any connection errors. If you run the server, while the server is available---it will receive data. Stop the server, and the client keeps going. It is still losing any data it collected while the server was down, but it's not crashing. Progress!

## How Are We Doing for Size?

Adding `thiserror` made hardly any difference. A release-compiled binary is 515,072 bytes on my machine. 503 kb. Not bad.

## Queueing and Resending Data

> The code for the library is in `code/05_server/shared_v2`. I'm breaking it into sections so you can see the progression.

It's probably not a great idea to simply drop any data we collect while the server is down. On the other hand, we don't want to keep it in memory forever, either. We need to queue it up, and then send it when the server is available. We'll keep a queue of data to send, and then send it when the server is available.

We're going to start by changing the `encode_v1` command to work with a *borrowed reference* to a command. We've been *moving* the data previously, because we didn't need to keep it. Now we do.

In the `shared` project's `lib.rs`:

```rust
pub fn encode_v1(command: &CollectorCommandV1) -> Vec<u8> {
```

This also lets you get rid of a `clone` call in the unit test:

```rust
let encoded = encode_v1(&command);
```

Now in `sender.rs` (in the `collector`), you can change the `send_command` function to accept a reference.

```rust
pub fn send_command(command: &CollectorCommandV1) -> Result<(), CollectorError> {
```

Finally, in `main` you can build a queue of commands to send and keep retrying:

```rust
// Listen for commands to send
let mut send_queue = VecDeque::with_capacity(120);
while let Ok(command) = rx.recv() {
    send_queue.push_back(command);

    // Send all queued commands
    while let Some(command) = send_queue.pop_front() {
        if sender::send_command(&command).is_err() {
            println!("Error sending command");
            send_queue.push_front(command);
            break;
        }
    }
}
```

This will keep retrying until the queue is empty.

Let's test it. Run the collector, wait until a few commands are pending and then run the server. It worked---data is enqueued, and when the server is sent it reaches the server.

But---we still have some bugs! Let's fix these:

* The collector is re-encoding every message, each time. That's the most expensive operation we have!
* The timestamp is generated during encoding, so when the data arrives---it tells you when the server came back. Not helpful.
* We're making a whole new connection for each message. Making the TCP connection is more expensive than using it, so we're wasting a bunch of time.

## Only Encoding Once (and fixing the timestamp)

So let's change our queue to store the *encoded* message:

```rust
// Listen for commands to send
let mut send_queue = VecDeque::with_capacity(120);
while let Ok(command) = rx.recv() {
    let encoded = shared_v2::encode_v1(&command);
    send_queue.push_back(encoded);
```

We'll also need to change our `send_command` function to accept a `&[u8]` instead of a `&CollectorCommandV1`:

```rust
use crate::errors::CollectorError;
use shared_v2::DATA_COLLECTOR_ADDRESS;
use std::io::Write;

pub fn send_command(bytes: &[u8]) -> Result<(), CollectorError> {
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS)
        .map_err(|_| CollectorError::UnableToConnect)?;
    stream
        .write_all(bytes)
        .map_err(|_| CollectorError::UnableToSendData)?;

    Ok(())
}
```

We accept a slice of bytes---no copy required. We've removed the `println`, which is surprisingly expensive. Otherwise, the function is the same; we're accepting the pre-encoded bytes and sending them.

Let's give that a test.

> `cargo build --release` shows that we're still weighing in at 517,120 bytes (505 kb). Not bad.

That solves the timestamp and encoding problems, but we're still making a new connection for each message. Let's fix that.

### Reusing Connections

Let's create a replacement for `send_command` that accepts a reference to the whole queue, and tries to send all of it on a single TCP connection:

```rust
pub fn send_queue(queue: &mut VecDeque<Vec<u8>>) -> Result<(), CollectorError> {
    // Connect
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS)
        .map_err(|_| CollectorError::UnableToConnect)?;

    // Send every queue item
    while let Some(command) = queue.pop_front() {
        if stream.write_all(&command).is_err() {
            queue.push_front(command);
            return Err(CollectorError::UnableToSendData);
        }
    }

    Ok(())
}
```

Now we can replace our sender in the `main` function with a much simpler:

```rust
// Listen for commands to send
let mut send_queue = VecDeque::with_capacity(120);
while let Ok(command) = rx.recv() {
    let encoded = shared_v2::encode_v1(&command);
    send_queue.push_back(encoded);
    let _ = sender::send_queue(&mut send_queue);
}
```

And now we're using a single TCP connection during the lifetime of the sender. That's much better.

> Why not keep a single TCP connection for the whole time? The server can only handle 64k connections on a single IP. Keeping a long-running connection per client will require more IP addresses if the widget is successful. You could also look at using UDP for submission. The downside of UDP is that you can lose the ability to reply (thanks to NAT).

There are still imperfections on the client-side, but let's start putting together the web service.