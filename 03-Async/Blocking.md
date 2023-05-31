# Blocking Tasks

We saw that you can stall the thread pool by doing computation---other tasks only get to run when you `yield`, `await` or otherwise give up control (by awaiting an IO task, for example).

Heavy CPU usage is "blocking" the task from yielding to other tasks. This is a problem if you want to do CPU intensive work in an async context.

## Sleeping

> The code for this example is in the `03_async\tokio_thread_sleep` directory.

Let's look at another way to wreak havoc! Add `tokio` (`cargo add tokio -F full`) and `futures` (`cargo add futures`) and run the following code:

```rust
use std::time::Duration;

async fn hello_delay(task: u64, time: u64) {
    println!("Task {task} has started");
    std::thread::sleep(Duration::from_millis(time));
    println!("Task {task} is done.");
}

#[tokio::main]
async fn main() {
    let mut futures = Vec::new();
    for i in 0..5 {
        futures.push(hello_delay(i, 500 * i));
    }
    futures::future::join_all(futures).await;
}
```

Despite using a multi-threaded runtime, and even though we've used `join_all`---the tasks run in-order with no sharing:

```
Task 0 has started
Task 0 is done.
Task 1 has started
Task 1 is done.
Task 2 has started
Task 2 is done.
Task 3 has started
Task 3 is done.
Task 4 has started
Task 4 is done.
```

This is happening because `std::thread::sleep` is blocking the whole thread, not just the task. It may even put the executor to sleep, if the task fires on the same thread as the executor---which is likely, because tasks never get the chance to be moved.

This is *worse* than high CPU usage blocking the task, because it can block the whole thread pool! With high CPU usage and a work-stealing pool, you'd normally expect the other tasks to be able to run on other threads.

Sleeping is a common requirement, and Tokio includes an async timing system for this reason. You can use Tokio's `sleep` instead to get what you expect:

```rust
use std::time::Duration;

async fn hello_delay(task: u64, time: u64) {
    println!("Task {task} has started");
    //std::thread::sleep(Duration::from_millis(time));
    tokio::time::sleep(Duration::from_millis(time)).await;
    println!("Task {task} is done.");
}

#[tokio::main]
async fn main() {
    let mut futures = Vec::new();
    for i in 0..5 {
        futures.push(hello_delay(i, 500 * i));
    }
    futures::future::join_all(futures).await;
}
```

Now the output looks reasonable:

```
Task 0 has started
Task 1 has started
Task 2 has started
Task 3 has started
Task 4 has started
Task 0 is done.
Task 1 is done.
Task 2 is done.
Task 3 is done.
Task 4 is done.
```

## But What if I Need to Block?

You might actually want to perform a blocking operation---I/O to a device that doesn't have an async interface, a CPU intensive task, or something else that can't be done asynchronously. Tokio implements `spawn_blocking` for this.

> The code for this is in `03_async\tokio_spawn_blocking`.

```rust
use std::time::Duration;
use tokio::task::spawn_blocking;

async fn hello_delay(task: u64, time: u64) {
    println!("Task {task} has started");
    let result = spawn_blocking(move || {
        std::thread::sleep(Duration::from_millis(time));
    }).await;
    println!("Task {task} result {result:?}");
    println!("Task {task} is done.");
}

#[tokio::main]
async fn main() {
    let mut futures = Vec::new();
    for i in 0..5 {
        futures.push(hello_delay(i, 500 * i));
    }
    futures::future::join_all(futures).await;
}
```

Notice that `spawn_blocking` returns a `Result`, containing whatever is returned from your blocking task.

Blocking tasks create a thread, and run the task on that thread. If you `await` it, your task will be suspended---and control given to other tasks---until its done. This way, you aren't blocking the tasks queue and can do your heavy lifting in a thread. The downside is that you have created a system thread, which is more expensive than a lightweight task.

If you specified a maximum number of blocking threads in your runtime builder, threads will wait until there is a free thread to run on. If you didn't, Tokio will create a new thread for each blocking task.

## What Happens if I Don't Await a Blocking Task?

If you just want your blocking task to "detach" (run independently) and neither block the current task nor return a result, you can use `spawn_blocking` without the `await`:

```rust
use std::time::Duration;
use tokio::task::spawn_blocking;

async fn hello_delay(task: u64, time: u64) {
    println!("Task {task} has started");
    spawn_blocking(move || {
        std::thread::sleep(Duration::from_millis(time));
    });
    println!("Task {task} is done.");
}

#[tokio::main]
async fn main() {
    let mut futures = Vec::new();
    for i in 0..5 {
        futures.push(hello_delay(i, 500 * i));
    }
    futures::future::join_all(futures).await;
}
```

This will lead to an instant output of:

```
Task 0 has started
Task 0 is done.
Task 1 has started
Task 1 is done.
Task 2 has started
Task 2 is done.
Task 3 has started
Task 3 is done.
Task 4 has started
Task 4 is done.
```

*Followed by a delay while the threads finish.*

This is useful is you want to do something in the background and don't need the result immediately. You can always store the result in a shared data structure or send it over a channel if you need it later.