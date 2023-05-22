# Sharing Data with Lock-Free Structures

We've covered using locking to safely share data, and atomics to safely share some data types without locks---but there's a third choice. Some data structures are "lock free", and can be shared between threads without locks or atomics. This is a very advanced topic---we'll touch on more of it in a couple of weeks. For now, let's look at a couple of pre-made crates that can help us share data without locks.

## DashMap and DashSet

If you have data that fits well into a `HashMap` or `HashSet`, `DashMap` is a great choice. It's a lock-free hash map that can be shared between threads. It has its own interior locking, and uses a "generational" system (similar to Java's garbage collector) for memory management. It's a great choice for a lot of use cases.

> See the `lockfree_map` project in the `code/02_threads` directory.

Let's add `dashmap` to our `Cargo.toml` with `cargo add dashmap`. We'll use `once_cell` again for initialization (`cargo add once_cell`)./

> This code is in the `lockfree_map` example in `code/02_threads`.

Then we'll write a program to use it:

```rust
use std::time::Duration;
use dashmap::DashMap;
use once_cell::sync::Lazy;

static SHARED_MAP: Lazy<DashMap<u32, u32>> = Lazy::new(DashMap::new);

fn main() {
    for n in 0..100 {
        std::thread::spawn(move || {
            loop {
                if let Some(mut v) = SHARED_MAP.get_mut(&n) {
                    *v += 1;
                } else {
                    SHARED_MAP.insert(n, n);
                }
            }
        });
    }

    std::thread::sleep(Duration::from_secs(5));
    println!("{SHARED_MAP:#?}");
}
```

This sleeps for 5 seconds while 100 threads insert or update data. There are no locks, and no atomics. It's all done with a lock-free data structure.

You can use `DashMap` and `DashSet` just like you would a regular `HashMap` and `HashSet`, with the exception that iterators are a little different. Instead of iterating on a tuple of `(key, value)`, you access just values and call the `key()` function to obtain the key.

Let's extend the example to show this:

```rust
for v in SHARED_MAP.iter() {
    println!("{}: {}", v.key(), v.value());
}
```