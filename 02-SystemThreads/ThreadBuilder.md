# The ThreadBuilder Pattern

> The code for this is in the `thread_builder` example, in the `code/02_threads` directory.

Sometimes, you want more control over the creation of a thread. Rust implements a builder pattern to help you with this.

Let's build a quick example:

```rust
use std::thread;

fn my_thread() {
    println!("Hello from a thread named {}", thread::current().name().unwrap());
}

fn main() {
    thread::Builder::new()
        .name("Named Thread".to_string())
        .stack_size(std::mem::size_of::<usize>() * 4)
        .spawn(my_thread).unwrap();
}
```

We've *named* the thread. This doesn't actually do much, but it's nice to have a name for the thread when debugging. You can reference the current thread name in log messages to help you figure out what's going on, and some debuggers will display the thread name.

We've also set the stack size. This is the amount of memory that the thread will have available for its stack. The default is 2MB, but you can set it to whatever you want. In this case, we've set it to 4 times the size of a pointer, which is 32 bits on a 32-bit system and 64 bits on a 64-bit system. This is a pretty small stack, but it's enough for this example.

Setting the stack size is useful if you are running a lot of threads, and want to reduce the memory overhead of each thread. If you have a lot of threads, and they're not doing much, you can reduce the stack size to save memory. If you don't allocate enough stack, you're thread will crash when you try to use more stack than you've allocated. Most of the time---you don't need to set this!