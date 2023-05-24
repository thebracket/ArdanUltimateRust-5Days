# Setting Thread Affinity

> This is the `thread_affinity` example in `code/02_threads`.

**Warning**: It's hotly debated whether you should do this! Common wisdom today is that it's usually better to let the OS scheduler determine where to run threads. Sometimes, though, you need to control where a thread runs.  For some high-performance code, it can help---you can avoid delays while data travels between CPUs.  For other code, it can hurt---you can cause delays while data travels between CPUs.  It's a trade-off, and you need to understand your code and your hardware to make the right decision.

Rust doesn't include native/standard-library source for setting thread affinity to a CPU. The mechanism varies by platform, and it's not a common task.

The `core_affinity` crate provides a relatively simple mechanism to set thread affinity. Let's add it:

```bash
cargo add core_affinity
```

Now, let's use it:

```rust
fn main() {
    let core_ids = core_affinity::get_core_ids().unwrap();

    let handles = core_ids.into_iter().map(|id| {
        std::thread::spawn(move || {
            // Pin this thread to a single CPU core.
            let success = core_affinity::set_for_current(id);
            if success {
                println!("Hello from a thread on core {id:?}");
            } else {
                println!("Setting thread affinity for core {id:?} failed");
            }
        })
    }).collect::<Vec<_>>();
    
    for handle in handles.into_iter() {
        handle.join().unwrap();
    }
}
```
