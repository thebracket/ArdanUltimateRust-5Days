# Scoped Threads

> The code for this is in `scoped_threads`, in the `code/02_threads` folder.

In the [previous example](./DividingWorkloads.md) we divided our workload into chunks and then took a copy of each chunk. That works, but it adds some overhead. Rust has a mechanism to assist with this pattern (it's a very common pattern): scoped threads.

Let's build an example:

```rust
use std::thread;

fn main() {
    const N_THREADS: usize = 8;

    let to_add: Vec<u32> = (0..5000).collect();
    let chunks = to_add.chunks(N_THREADS);
    let sum = thread::scope(|s| {
        let mut thread_handles = Vec::new();

        for chunk in chunks {
            let thread_handle = s.spawn(move || {
                let mut sum = 0;
                for i in chunk {
                    sum += i;
                }
                sum
            });
            thread_handles.push(thread_handle);
        }

        thread_handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .sum::<u32>()
    });
    println!("Sum is {sum}");
}
```

This is quite similar to the previous example, but we're using *scoped threads*. When you use `thread::scope` you are creating a *thread scope*. Any threads you spawn with the `s` parameter are *guaranteed* to end when the scope ends. You can still treat each scope just like a thread.

Because the threads are *guaranteed* to terminate, you can safely borrow data from the parent scope. This is a *lifetime* issue: a normal thread could keep running for a long time, past the time the scope that launched it ends---so borrowing data from that scope would be a bug (and a common cause of crashes and data corruption in other languages). Rust won't let you do that. But since you have the guarantee of lifetime, you can borrow data from the parent scope without having to worry about it.

This pattern is perfect for when you want to fan out a workload to a set of calculation threads, and wait to combine them into an answer.