# Returning Data from Threads

> See the code `thread_return` in `code/02_threads`.

The thread handle will return any value returned by the thread. It's generic, so it can be of any type (that supports sync+send; we'll cover that later). Each thread has its own stack, and can make normal variables inside the thread---and they won't be affected by other threads.

Let's build an example:

```rust
fn do_math(i: u32) -> u32 {
    let mut n = i+1;
    for _ in 0 .. 10 {
        n *= 2;
    }
    n
}

fn main() {
    let mut thread_handles = Vec::new();
    for i in 0..10 {
        thread_handles.push(std::thread::spawn(move || {
            do_math(i)
        }));
    }

    for handle in thread_handles {
        println!("Thread returned: {}", handle.join().unwrap());
    }
}
```

This returns:

```
Thread returned: 1024
Thread returned: 2048
Thread returned: 3072
Thread returned: 4096
Thread returned: 5120
Thread returned: 6144
Thread returned: 7168
Thread returned: 8192
Thread returned: 9216
Thread returned: 10240
```

Notice that each thread is doing its own math, and returning its own value. The `join` function waits for the thread to finish, and returns the value from the thread.