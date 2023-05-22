# Spawning Threads with Parameters

> This uses the `thread_closures` code, in `code/02_threads`.

The `spawn` function takes a function without parameters. What if we want to pass parameters to the thread? We can use a closure:

```rust
fn hello_thread(n: u32) {
    println!("Hello from thread {n}!");
}

fn main() {
    let mut thread_handles = Vec::new();
    for i in 0 .. 5 {
        let thread_handle = std::thread::spawn(move || hello_thread(i));
        thread_handles.push(thread_handle);
    }
    thread_handles.into_iter().for_each(|h| h.join().unwrap());
}
```

Notice three things:

* We're using a *closure*---an inline function that can capture variables from the surrounding scope.
* We've used the shorthand format for closure: `|| code` - parameters live in the `||` (there aren't any), and a single statement goes after the `||`. You can use complex closures with a scope: `|x,y| { code block }`.
* The closure says `move`. Remember when we talked about ownership? You have to *move* variables into the closure, so the closure gains ownership of them. The ownership is then passed to the thread. Otherwise, you have to use some form of synchronization to ensure that data is independently accessed---to avoid race conditions.

The output will look something like this (the order of the threads will vary):

```
Hello from thread 0!
Hello from thread 2!
Hello from thread 1!
Hello from thread 4!
Hello from thread 3!
```

In this case, as we talked about last week in [Rust Fundamentals](../01-GettingStarted/RustFundamentals.md) integers are *copyable*. So you don't have to do anything too fancy to share them.