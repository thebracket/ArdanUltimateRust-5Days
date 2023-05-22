# Sending Functions to Threads

We've focused on sending commands indicating that there's work to do. But what about sending whole functions? We can do that too!

> The code for this is in `sending_functions` in the `code/02_threads` folder.

```rust
use std::sync::mpsc;

type Job = Box<dyn FnOnce() + Send + 'static>;

fn hi_there() {
    println!("Hi there!");
}

fn main() {
    let (tx, rx) = mpsc::channel::<Job>();
    let handle = std::thread::spawn(move || {
        while let Ok(job) = rx.recv() {
            job();
        }
    });

    let job = || println!("Hello from the thread!");
    let job2 = || {
        for i in 0..10 {
            println!("i = {i}");
        }
    };
    tx.send(Box::new(job)).unwrap();
    tx.send(Box::new(job2)).unwrap();
    tx.send(Box::new(hi_there)).unwrap();
    tx.send(Box::new(|| println!("Inline!"))).unwrap();
    handle.join().unwrap();
}
```

There's a bit to unwrap here:

* What is a `Box`? A `Box` is a smart pointer to an area of the heap. So the function pointer is placed inside a smart pointer, and then sent to the thread. The thread then takes ownership of the smart pointer, and when it's done with it, the smart pointer is dropped, and the function pointer is dropped with it. Without a box, you run into lifetime issues. You'll learn all about Boxes in a couple of weeks.
* What about `dyn`? `dyn` is a special marker indicating that the contents is dynamic. In this case, it's a dynamic function pointer. It doesn't necessarily point to just one function, and the exact form of the function is dynamic.
* How about `FnOnce`? This is a function that indicates that it will run once, and won't try to change the world around it. You quickly get into trouble when you need scope capture when you are passing function pointers around.
* What about `Send`? This indicates that the function pointer can be sent to another thread. This is a special marker trait that indicates that the function pointer is safe to send to another thread.

## Send and Sync

Types in Rust can implement the `Sync` and `Send` traits. `Sync` means that a type is synchronized, and can be safely modified. `Mutex` is a good example of a `Sync` type. `Send` means that a type can be sent to another thread. In this case, we're requiring that the function can be safely sent between threads.

Most of the time, `Sync` and `Send` are figured out for you. If anything in a structure isn't sync or send, the structure won't be. (You can override it if you really need to!)

If you're moving data along channels and run into a `Sync` or `Send` error it's usually a clue that you need to add protection---like a Mutex---around the data.

We'll look at `Send` and `Sync` later.

## Sending Commands and Functions

You can mix and match commands and functions using the same channel. Rust Enumerations can hold functions, just like any other type. Let's fix the issue of the program never quitting:

> This is the `sending_commands_and_functions` example in `code/02_threads`.

```rust
use std::sync::mpsc;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Command {
    Run(Job),
    Quit,
}

fn main() {
    let (tx, rx) = mpsc::channel::<Command>();
    let handle = std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::Run(job) => job(),
                Command::Quit => break,
            }
        }
    });

    let job = || println!("Hello from the thread!");
    tx.send(Command::Run(Box::new(job))).unwrap();
    tx.send(Command::Quit).unwrap();
    handle.join().unwrap();
}
```

