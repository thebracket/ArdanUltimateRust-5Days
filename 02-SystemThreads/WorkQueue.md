# Let's Build a Work Queue with a Thread Pool

> This is the `work_queue` example in `code/02_threads`.

Curiously, Rust doesn't have an easy cross-platform way to determine how many CPUs you have. So we'll add the `num_cpus` crate to help. We also want to use `once_cell` again: 

```
cargo add num_cpus
cargo add once_cell
```

## Imports/Use

We're going to start with some imports:

```rust
use std::{sync::Mutex, collections::VecDeque, time::Duration};
use once_cell::sync::Lazy;
```

`Mutex` and `Duration` we've used before. `VecDeque` is another built-in collection: a queue, implemented using a vector as backing.

You can treat a `VecDeque` like a queue in other languages: pushing to the back, popping from the front. It's a good data-structure to represent a queue of work to be done.

## Setting up the work queue

We'll setup the work queue just like other shared, lazy variables:

```rust
static WORK_QUEUE: Lazy<Mutex<VecDeque<String>>> = Lazy::new(|| Mutex::new(VecDeque::new()));
```

We're just storing a string for the job---but it can be anything you like.

## Setting up the threads

We find out out many CPUs we have with the `num_cpus` crate:

```rust
let cpu_count = num_cpus::get();
```

We can use that to size two vectors we're going to need:

```rust
let mut threads = Vec::with_capacity(cpu_count);
let mut broadcast = Vec::with_capacity(cpu_count);
```

We're going to use "broadcast" as a set of channels that send to all the threads.

## Build the Threads

Now let's build our threads:

```rust
for cpu in 0..cpu_count {
    let (tx, rx) = std::sync::mpsc::channel::<()>();
    broadcast.push(tx);

    let thread = std::thread::spawn(move || {
        while rx.recv().is_ok() {
            let mut lock = WORK_QUEUE.lock().unwrap();
            if let Some(work) = lock.pop_front() {
                std::mem::drop(lock);
                println!("CPU {cpu} got work: {work}");
                std::thread::sleep(Duration::from_secs(2));
                println!("CPU {cpu} finished!");
            } else {
                println!("CPU {cpu} found no work");
            }
        }

    });
    threads.push(thread);
}
```

For each CPU, we create an MPSC channel---with a sender and receiver. We add the sender to the broadcast vector, and move the receiver into the thread. We're using a dirty trick here: the channel is typed on `()` - the unit type. It contains no data---we just care that there *is* a message.

The thread locks the Mutex, and uses `pop_front` to see if there's a work message. If there is, it drops the lock explicitly---with `std::mem::drop`. This is a trick to make sure the lock is dropped before we do the work. If we didn't do this, the lock would be held while we did the work, and no other thread could get the lock.

We then simulate doing something by sleeping. That works in the office too, sometimes.

## Ordering Some Work

Finally, we order some work. We'll keep the work queue from growing forever:

```rust
loop {
    let sent = {
        let mut lock = WORK_QUEUE.lock().unwrap();
        let len = lock.len();
        println!("There are {len} items in the queue");
        if len < 5 {
            lock.push_back("Hello".to_string());
            true
        } else {
            false
        }
    };
    if sent {
        broadcast.iter().for_each(|tx| tx.send(()).unwrap());
    }
    std::thread::sleep(Duration::from_secs(1));
}
```

## Full Example

Here's the full example:

```rust
use std::{sync::Mutex, collections::VecDeque, time::Duration};
use once_cell::sync::Lazy;

static WORK_QUEUE: Lazy<Mutex<VecDeque<String>>> = Lazy::new(|| Mutex::new(VecDeque::new()));

fn main() {
    // Commented out for clarity: a real work pool will use this
    //let cpu_count = num_cpus::get();
    let cpu_count = 2;
    let mut threads = Vec::with_capacity(cpu_count);
    let mut broadcast = Vec::with_capacity(cpu_count);


    for cpu in 0..cpu_count {
        let (tx, rx) = std::sync::mpsc::channel::<()>();
        broadcast.push(tx);

        let thread = std::thread::spawn(move || {
            while rx.recv().is_ok() {
                let mut lock = WORK_QUEUE.lock().unwrap();
                if let Some(work) = lock.pop_front() {
                    std::mem::drop(lock);
                    println!("CPU {cpu} got work: {work}");
                    std::thread::sleep(Duration::from_secs(5));
                    println!("CPU {cpu} finished!");
                } else {
                    println!("CPU {cpu} found no work");
                }
            }

        });
        threads.push(thread);
    }

    loop {
        let sent = {
            let mut lock = WORK_QUEUE.lock().unwrap();
            let len = lock.len();
            println!("There are {len} items in the queue");
            if len < 5 {
                lock.push_back("Hello".to_string());
                true
            } else {
                false
            }
        };
        if sent {
            broadcast.iter().for_each(|tx| tx.send(()).unwrap());
        }
        std::thread::sleep(Duration::from_secs(1));
    }
}
```


