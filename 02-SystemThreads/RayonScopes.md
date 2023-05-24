# Rayon Scopes and Thread Pools

> The code for this is in the `rayon_scopes` directory in the `code/02_threads` directory.

Don't forget to add Rayon with `cargo add rayon`.

Remember [Scoped Threads](./ScopedThreads.md)? Rayon can help with that, too. Rayon can use a default thread pool (with one thread per CPU), but often you want to specifically limit the number of threads used by part of your application. Let's build some code that builds a limited thread pool, runs a thread in the pool, and then uses a "scope" to execute some more tasks with the same lifetime relaxation as the scoped thread example:

```rust
fn main() {
    // Let's explicitly size our thread pool
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.spawn(|| println!("Hello from pool thread"));

    pool.scope(|scope| {
        for n in 0..20 {
            scope.spawn(move |_| {
                println!("Hello from thread {n}");
            });
        }
    });
    println!("Hello from main thread");
}
```

The output of the thread numbers will appear somewhat random---the order of execution is not guaranteed.

## Multiple Pools

You can spawn thread pools wherever you need them. The pools are independent of each other. Note that you can also spawn more workers inside the same scope---which is usually a better idea.

> The code for this is in the `rayon_nested_pools` directory in the `code/02_threads` directory.

```rust
fn main() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    // We're using a scope to ensure that we wait for everything to finish
    pool.scope(|scope| {
        for n in 0..4 {
            scope.spawn(move |_scope | {
                println!("Hello from top-level {n}");
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                
                pool.scope(|scope| {
                    for inner_n in 0.. 4 {
                        scope.spawn(move |_scope| {
                            println!("Hello from inner {inner_n} (part of {n})");
                        });
                    }
                });
                
                println!("Goodbye from top-level {n}");
            });
        }
    });
}
```

## A Couple of Other Handy Rayon Features

There's a couple more handy things Rayon can do.

### Broadcast: Every thread runs a function

> The code for this is in the `rayon_broadcast` directory in the `code/02_threads` directory.

If you want to run the same function on as many threads as you have available, you can use `spawn_broadcast`:

```rust
fn main() {
    // Let's explicitly size our thread pool
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.scope(|scope| {
        scope.spawn_broadcast(|_scope, broadcast_context| {
            // You can use scope just like the parent scope
            println!("Hello from broadcast thread {}", broadcast_context.index());
        });
    });
}
```

### Join: Spawn several tasks and wait for all of them

> The code for this is in the `rayon_join` directory in the `code/02_threads` directory.

Rayon doesn't `join` the way the standard library does. Instead of receiving a handle and waiting for it, `join` runs exactly two tasks at once and waits for both to finish.

```rust
fn test() {
    println!("Hello from test thread");
}

fn main() {
    // Let's explicitly size our thread pool
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.join(test, test);
}
```