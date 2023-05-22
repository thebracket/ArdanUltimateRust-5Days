# Create Your First Thread

> This uses the `first_thread` code, in `code/02_threads`.

## Create a new project - with a workspace

Looking back at the [workspaces](../01-GettingStarted/Workspaces.md) class from last week, it's a great idea to have a workspace. Let's create one:

```bash
cargo new LiveWeek2
```

Now edit `Cargo.toml` to include a workspace:

```toml
[workspace]
members = []
```

Now change directory to the `LiveWeek2` directory and create a new project named `FirstThread`:

```bash
cd LiveWeek2
cargo new FirstThread
```

And add the project to the workspace:

```toml
[workspace]
members = [
    "FirstThread"
]
```

## Your First Thread

In `main.rs`, replace the contents with the following:

```rust
fn hello_thread() {
    println!("Hello from thread!");
}

fn main() {
    println!("Hello from main thread!");

    let thread_handle = std::thread::spawn(hello_thread);
    thread_handle.join().unwrap();
}
```

Now run the program:

```bash
Hello from main thread!
Hello from thread!
```

So what's going on here? Let's break it down:

1. The program starts in the main thread.
2. The main thread prints a message.
3. We create a thread using `std::thread::spawn` and tell it to run the function `hello_thread`.
4. The return value is a "thread handle". You can use these to "join" threads---wait for them to finish.
5. We call `join` on the thread handle, which waits for the thread to finish.

### What happens if we don't join the thread?

Run the program a few times. Sometimes the secondary thread finishes, sometimes it doesn't. Threads don't outlive the main program, so if the main program exits before the thread finishes, the thread is killed.