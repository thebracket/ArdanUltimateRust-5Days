# Parking Threads

> The code for this is in `parking`, in the `code/02_threads` directory.

We've talked about how advantageous it can be to create a thread and reuse it. Sometimes, you want to run a threaded task occasionally---but still in the background---and with as little latency as possible. One way to do this is to "park" the thread, which means to put it to sleep until it's needed again. Parked threads consume almost no CPU time, and can be woken up very quickly.

Let's build an example of parking threads:

```rust
fn read_line() -> String {
    // <- Public function
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn parkable_thread(n: u32) {
    loop {
        std::thread::park();
        println!("Thread {n} is awake - briefly!");
    }
}

fn main() {
    let mut threads = Vec::new();
    for i in 0..10 {
        let thread = std::thread::spawn(move || parkable_thread(i));
        threads.push(thread);
    }

    loop {
        println!("Enter a thread number to awaken, or q to quit");
        let input = read_line();
        if input == "q" {
            break;
        }
        if let Ok(number) = input.parse::<u32>() {
            if number < threads.len() as u32 {
                threads[number as usize].thread().unpark();
            }
        }
    }
}
```

Notice that any thread can call `park`, and suspend the currently executing thread. In this example, we park 10 threads and let the user choose to wake them up from the keyboard.

This can be very useful if you have a monitor going that detects an event, and wakes up the relevant thread when it detects that the thread is needed. It has the downside that you aren't sending any *data* to the thread. We'll talk about that next.