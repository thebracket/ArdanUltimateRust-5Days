# Async Channels

You should remember from [last week](../02-SystemThreads/Channels.md) that we talked about MPSC channels for communicating between threads. If not, I've linked to the curriculum and you have the code we created.

## Sync Channels

You can use threaded channels. A common approach for batch processing is to use an `async` system to receive network request, and then send them to dedicated system threads for heavy processing.

> The code for this section is in `03_async/sync_channel`.

Don't forget to add Tokio!

This provides a very convenient way to send data *into* a system that is using system threads:

```rust
use std::{time::Duration, sync::mpsc};

enum Command {
    Print(String),
}

#[tokio::main]
async fn main() {
    // Spawn a command thread for "heavy lifting"
    let (tx, rx) = mpsc::channel::<Command>();

    std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::Print(s) => println!("{s}"),
            }
        }
    });

    // Launch the async sender
    let mut counter = 0;
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        tx.send(Command::Print(format!("Hello {counter}"))).unwrap();
        counter += 1;
    }
}

```

You are creating a threaded process that waits for commands, and prints them out. You also create an async runtime, and repeatedly send commands to the threaded process.

This isn't very useful---but it can be connected to a network server that has to do heavy batch processing, and suddenly you have the best of both worlds: your threaded task is doing the heavy lifting. You could even use Rayon with a limited-size thread pool to control how many cores you are using, and reserve some (even one) for Tokio.

## Replying to Sync Channels

But what about getting the result back into the Async world?

> The code for this is in `03_async/sync_channel_reply`.

Tokio *also* implements MPSC channels. They behave a lot like their treaded brethren, but they are async. Sending requires an `await`, receiving requires an `await`. They are very efficient on the async side of the fence.

Now here's a neat trick.

Once you have a Tokio runtime, you can get a *handle* to it at any time and use that inside synchronous code to launch async tasks inside the executor!

This lets you bridge the divide between the threaded world and the async world. You can indeed have your cake and eat it.

Starting with the previous example (copied into a new entry in the work repo), we add a Tokio channel for replies *back into the async world*:

```rust
// Spawn a TOKIO Async channel for replies
let (tx_reply, mut rx_reply) = tokio::sync::mpsc::channel::<String>(10);
```

This is exactly like creating a threaded version, but we're using the Tokio variant. Tokio also requires that channels be *bounded*---the number of messages that can sit in the queue awaiting processing. That's the `10`.

Now we can modify our system thread to obtain a handle to Tokio, and use it to spawn an async reply:

```rust
let handle = tokio::runtime::Handle::current();
std::thread::spawn(move || {
    while let Ok(command) = rx.recv() {
        match command {
            Command::Print(s) => {
                // Make our very own copy of the transmitter
                let tx_reply = tx_reply.clone();
                handle.spawn(async move {
                    tx_reply.send(s).await.unwrap();
                });
            },
        }
    }
});
```

Lastly, we add an async process (running in the background) to receive the replies:

```rust
// Launch a Tokio process to receive replies from thread-land
tokio::spawn(async move {
    while let Some(reply) = rx_reply.recv().await {
        println!("{reply}");
    }
});
```

Here's the full code:

```rust
use std::{time::Duration, sync::mpsc};

enum Command {
    Print(String),
}

#[tokio::main]
async fn main() {
    // Spawn a command thread for "heavy lifting"
    let (tx, rx) = mpsc::channel::<Command>();

    // Spawn a TOKIO Async channel for replies
    let (tx_reply, mut rx_reply) = tokio::sync::mpsc::channel::<String>(10);

    let handle = tokio::runtime::Handle::current();
    std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::Print(s) => {
                    // Make our very own copy of the transmitter
                    let tx_reply = tx_reply.clone();
                    handle.spawn(async move {
                        tx_reply.send(s).await.unwrap();
                    });
                },
            }
        }
    });

    // Launch a Tokio process to receive replies from thread-land
    tokio::spawn(async move {
        while let Some(reply) = rx_reply.recv().await {
            println!("{reply}");
        }
    });

    // Launch the async sender
    let mut counter = 0;
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        tx.send(Command::Print(format!("Hello {counter}"))).unwrap();
        counter += 1;
    }
}
```

It runs as before, but you've got a really good template here:

* You spawn system threads, using everything you learned [last week](../02-SystemThreads/README.md).
* Since system threads are perfect for CPU-bound workloads, you don't have to worry about yielding, spawning blocking tasks, or anything like that. You just receive a message telling you to do something, and you hit it as hard as you can.
* Meanwhile, Tokio remains entirely async---giving fast network or other IO access.

This is a popular pattern for batch processing. Another service tells your program (often over the network, but it could be a channel or anything else) that there's some heavy processing ready to do. You send the CPU-bound workload off into a thread pool (often using Rayon) and send a message back when it is done.

## Tokio Broadcast Channels

> The code for this is in `03_async/broadcast`.

Tokio provides a type of channel that regular Rust doesn't have: the broadcast channel. This is a channel that can have multiple receivers. It's a bit like a `Vec` of channels, but it's more efficient. It's relatively easy to use:

```rust
#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::broadcast::channel::<String>(16);

    for n in 0..16 {
        let mut messages = tx.subscribe();
        tokio::spawn(async move {
            while let Ok(msg) = messages.recv().await {
                println!("{n}: {msg}");
            }
        });
    }

    tx.send("hello".to_string()).unwrap();
    while let Ok(msg) = rx.recv().await {
        println!("main: {msg}");
    }
}
```

This example will never terminate! But if you need to send a message to a lot of tasks at once, this is a great way to do it.