# Channels and Ownership

Channels are an easy way to send data between threads, but ownership becomes a question.

Trying to pass a reference into a channel becomes problematic fast. Unless you can guarantee that the calling thread will outlive the data---and retain the data in a valid state---you can't pass a reference. The "lifetime checker" part of the borrow checker will complain.

The easiest approach is to **move** the data. The data arrives in one thread, which owns it. Rather than cloning, we *move* the data into the channel. The channel then owns the data, and can move it to another thread if needs-be. There's never any question of ownership, it's always clear who owns the data.

Let's look at an example:

```rust
use std::sync::mpsc;

// Not copyable or clone-able
struct MyData {
    data: String,
    n: u32,
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    let (tx, rx) = mpsc::channel::<MyData>();

    std::thread::spawn(move || {
        while let Ok(data) = rx.recv() {
            println!("--- IN THE THREAD ---");
            println!("Message number {}", data.n);
            println!("Received: {}", data.data);
        }
    });

    let mut n = 0;
    loop {
        println!("Enter a string");
        let input = read_line();
        let data_to_move = MyData {
            data: input,
            n,
        };
        n += 1;

        tx.send(data_to_move).unwrap();
    }
}
```

This pattern is also fast. Moving data generates a `memcpy` command behind the scenes, but most of the time the optimizer is able to remove it.

Let's benchmark it:

```rust
use std::sync::mpsc;

// Not copyable or clone-able
struct MyData {
    start: std::time::Instant,
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    let (tx, rx) = mpsc::channel::<MyData>();

    std::thread::spawn(move || {
        while let Ok(data) = rx.recv() {
            let elapsed = data.start.elapsed();
            println!("--- IN THE THREAD ---");
            println!("Message passed in {} us", elapsed.as_micros());
        }
    });

    loop {
        println!("Enter a string");
        let _ = read_line();
        let data_to_move = MyData {
            start: std::time::Instant::now(),
        };

        tx.send(data_to_move).unwrap();
    }
}
```

On my development box, it averages 17 us per message. That's pretty fast. Definitely enough that if you are doing some serious work, you can afford to move the data.