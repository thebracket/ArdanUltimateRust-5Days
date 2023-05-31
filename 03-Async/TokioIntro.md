# Getting Started with Tokio

Tokio is an async executor for Rust, that implements just about everything you need for Enterprise usage. It is a bit of a kitchen-sink project, tamed by using multiple optional dependencies. It is also the most popular async executor for Rust.

## Single-Threaded Usage

Tokio supports multiple threads, but can be used as a single-threaded executor. In some cases, this is what you want---if the bulk of your system is taken up by CPU intensive tasks, you may only want to dedicate some resources to running async tasks. You may also want to use Tokio as a single-threaded executor for testing purposes, or for tools that need to be kept small.

> See the `03_async\tokio_single_thread_manual` code example.

To use the Tokio executor, you need to add it to your project. It supports a lot of options; for now, we'll use the "full" feature to get access to everything:

```bash
cargo add tokio -F full
```

Let's build a very simple single-threaded async app:

```rust
use tokio::runtime;

async fn hello() {
    println!("Hello from async");
}

fn main() {
    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(hello());
}
```

Notice that this is just like the code we created for `futures`---only using a different runtime. Under the hood, async programs always have a top-level `block_on` or equivalent to hosts the async session.

You don't have to only use Tokio! You can spawn threads before you block on the async session, and use them for CPU intensive tasks. You can even run executors in threads and have multiple async sessions running independently (or communicating through channels).

## Let's Make It Easier

> We're switching to the `03_async/tokio_single_thread_macro` example code.

Tokio includes some helper macros to avoid typing the boilerplate every time. If you don't need very specific control over the executor, you can use the `#[tokio::main]` macro to create a single-threaded executor:

```rust
async fn hello() {
    println!("Hello from async");
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    hello().await;
}
```

That's reduced your code size down to 8 lines! The `#[tokio::main]` macro will create a Tokio runtime, and block on the async session you provide. We've added a `flavor = "current_thread"` parameter to ask Tokio to run in a single thread.

## Multi-threaded Tokio - the long form with options

Tokio can also run in multi-threaded mode. It's very sophisticated:

* It spawns one thread per CPU by default---you can control this.
* Each thread has its own "task list".
* Each thread has its own "reactor" (event loop).
* Each thread supports "work stealing"---if the thread has nothing to do, and other threads are blocking on a task, they can "steal" tasks from other threads. This makes it harder to stall your program.
* You can configure the number of threads, and the number of "reactors" (event loops) per thread.

You usually don't need to set all the options, but here's a set of what you can change if you need to:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::runtime;

async fn hello() {
    println!("Hello from async");
}

fn thread_namer() -> String {
    static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
    let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
    format!("my-pool-{id}")
}

fn main() {
    let rt = runtime::Builder::new_multi_thread()
        // YOU DON'T HAVE TO SPECIFY ANY OF THESE
        .worker_threads(4)  // 4 threads in the pool
        .thread_name_fn(thread_namer) // Name the threads. 
                                     // This helper names them "my-pool-#" for debugging assistance.
        .thread_stack_size(3 * 1024 * 1024) // You can set the stack size
        .event_interval(61) // You can increase the I/O polling frequency
        .global_queue_interval(61) // You can change how often the global work thread is checked
        .max_blocking_threads(512) // You can limit the number of "blocking" tasks
        .max_io_events_per_tick(1024) // You can limit the number of I/O events per tick
        // YOU CAN REPLACE THIS WITH INDIVIDUAL ENABLES PER FEATURE
        .enable_all()
        // Build the runtime
        .build()
        .unwrap();

    rt.block_on(hello());
}
```

In other words, if you *need* the control. It's there. Most of the time, you'll not need to change any of this. Just like single-threaded, you can mix/match with system threads and multiple executors if you need to (multiple executors can get messy!).

## Tokio Multi-Threaded - Macro Style

That's a lot of boilerplate. If you don't need to reconfigure how everything works. This makes for a very simple,readable `main.rs`:

```rust
async fn hello() {
    println!("Hello from async");
}

#[tokio::main]
async fn main() {
    hello().await;
}
```