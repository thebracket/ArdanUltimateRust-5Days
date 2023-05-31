# Selecting Futures

There's one more type of spawning option to consider. It's a complicated one, so it got its own section. It's called `select!`. It's a macro that lets you wait for the first of several futures to complete---and then automatically cancels the other futures.

## Implementing Timeouts

> The code for this is in `03_async/select_timeout`.

Let's start with a simple example. We'll spawn two futures. One will sleep for 1 second, and the other will sleep for 2 seconds. We'll use `select!` to wait for the first one to complete. Then we'll print a message and exit.

```rust
use std::time::Duration;
use tokio::time::sleep;

async fn do_work() {
    // Pretend to do some work that takes longer than expected
    sleep(Duration::from_secs(2)).await;
}

async fn timeout(seconds: f32) {
    // Wait for the specified number of seconds
    sleep(Duration::from_secs_f32(seconds)).await;
}

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = do_work() => println!("do_work() completed first"),
        _ = timeout(1.0) => println!("timeout() completed first"),
    }
}
```

The syntax is based on a `match` statement, but with an additional step. The format is:
`(value) = (function) => (what to do if it finishes first)`

You can change the timeout to determine which will finish first. If you set it to 3 seconds, then `do_work()` will finish first. If you set it to 0.5 seconds, then `timeout()` will finish first.

> Note that the other future is cancelled---but if you've done work that has side-effects (say saving to a file) the work that has already been performed will not be undone.

## Receiving from Multiple Channels

> The code for this is in `03_async/select_channels`.

An easy way for an async function to be subscribed to multiple channels is to obtain the receivers, and then `select!` whichever one has data. Here's an example:

```rust
use tokio::sync::{mpsc, broadcast};

async fn receiver(mut rx: mpsc::Receiver<u32>, mut broadcast_rx: broadcast::Receiver<u32>) {
    loop {
        tokio::select! {
            Some(n) = rx.recv() => println!("Received message {n} on the mpsc channel"),
            Ok(n) = broadcast_rx.recv() => println!("Received message {n} on the broadcast channel"),
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<u32>(1);
    let (broadcast_tx, broadcast_rx) = broadcast::channel::<u32>(1);

    tokio::spawn(receiver(rx, broadcast_rx));

   for count in 0 .. 10 {
        if count % 2 == 0 {
            tx.send(count).await.unwrap();
        } else {
            broadcast_tx.send(count).unwrap();
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
```

Note that if you have a continuous stream in the MPSC channel, the broadcast channel may take a *while* to fire! This pattern is good for sending "quit" messages and other control data---but only if it doesn't have to be instantaneous.
