# Sending Data Between Threads with Channels

[Parking a thread](./ParkingThreads.md) is great, but you often need to tell a thread *why* you woke it up, or give it some data to work with. This is where channels come in.

If you're used to Go, channels should sound familiar. They are very similar to Go's channels. A few differences:

* Rust Channels are strongly typed. So you can use a sum type/enum to act like a command pattern.
* Rust Channels are bounded by size, and will block if you try to send data to a full channel.
* Rust Channels are unidirectional. You can't send data back to the sender. (You can make another channel)
* You can't forget to close a channel. Once a channel is out of scope, the "drop" system (we'll talk about that in a couple of weeks) will close the channel for you.

## Multi-Producer, Single Consumer Channels

> See the `mpsc` project in the `code/02_threads` directory.

The most basic type of channel is the MPSC channel: any number of producers can send a message to a single consumer. Let's build a simple example:

```rust
use std::sync::mpsc;

enum Command {
    SayHello, Quit
}

fn main() {
    let (tx, rx) = mpsc::channel::<Command>();

    let handle = std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::SayHello => println!("Hello"),
                Command::Quit => {
                    println!("Quitting now");
                    break;
                }
            }
        }
    });

    for _ in 0 .. 10 {
        tx.send(Command::SayHello).unwrap();
    }
    println!("Sending quit");
    tx.send(Command::Quit).unwrap();
    handle.join().unwrap();
}
```

This is a relatively simple example. We're only sending messages to one thread, and not trying to send anything back. We're also not trying to send anything beyond a simple command. But this is a great pattern---you can extend the `Command` to include lots of operations, and you can send data along with the command. Threads can send to other threads, and you can `clone` the `tx` handle to have as many writers as you want.

We're going to build on the channel system after the break.