# Sharing Data with Mutexes

> The code for this is in the `mutex` example, in the `code/02_threads` folder.

So we just saw that using "unsafe" is a bad idea. Atomics are wonderful, but they only work for types that have built-in atomic operations. What if we want to share a `String` or a `Vec` between threads? We can't use an atomic for that---there is no atomic `String` or `Vec` type.

A `Mutex` is like a traffic signal: when you "lock" a mutex, you either gain access to the data, or you wait until the data is available. When you're done with the data, you "unlock" the mutex, and the next thread can access the data. Unlocking makes use of Rust's scoping system---the lock will be automatically relinquished when the lock leaves scope.

Let's build a simple example using a `Mutex`:

```rust
use std::sync::Mutex;

static NUMBERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

fn main() {
    let mut thread_handles = Vec::new();
    for _ in 0..10 {
        let handle = std::thread::spawn(|| {
            for _ in 0..10_000 {
                let mut lock = NUMBERS.lock().unwrap();
                lock.push(1);
            }
        });
        thread_handles.push(handle);
    }

    thread_handles.into_iter().for_each(|h| h.join().unwrap());

    let lock = NUMBERS.lock().unwrap();
    println!("Numbers length: {}", lock.len());
}
```

Notice that we haven't used `once_cell` or similar to initialize the static. Creating a `Mutex` and an empty `Vec` are *constant* functions. You don't have to do anything special to initialize them---they can be safely constructed at compile time.

## Performance of Mutexes

Mutexes are quite fast. On Linux, they are backed by a kernel-side "futex", which is one of the fastest locking systems ever created. You *are* experiencing delays when you lock and unlock a mutex, but they are very small delays.

> See the `mutex_timed` code in the `code/02_threads` folder for a demonstration of how fast mutexes are.

We've added a `mutex_locked` function to the timing code:

```rust
fn mutex_locked() {
    let mut handles = Vec::new();
    for _ in 0..1_000 {
        let handle = std::thread::spawn(|| {
            for _ in 0..10_000 {
                *MUTEX_COUNTER.lock().unwrap() += 1;
            }
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("Mutex: {}", *MUTEX_COUNTER.lock().unwrap());
}
```

The timing results show that a Mutex is a *lot* slower than an atomic:

```
Timing Results:
Unsafe: 0.031093 seconds
Atomic: 0.1301053 seconds
Mutex: 1.0850008 seconds
```

So the lesson here is that you only want to use a `Mutex` when need to lock a structure that can't be atomically updated. If you can use an atomic, you should. Note that we're deliberately hitting the *worst* case for a Mutex here---there's almost guaranteed to be contention.

Let's try a version that does the incrementing, and then locks the `Mutex` once and stores the result:

```rust
fn smarter_mutex_locked() {
    let mut handles = Vec::new();
    for _ in 0..N_THREADS {
        let handle = std::thread::spawn(|| {
            let mut n = 0;
            for _ in 0..N_ITERATIONS {
                n += 1;
            }
            *MUTEX_COUNTER2.lock().unwrap() += n;
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("Smarter Mutex            : {}", *MUTEX_COUNTER2.lock().unwrap());
}
```

And the results:

```
Timing Results:
Unsafe:        0.03 seconds
Atomic:        0.13 seconds
Mutex:         1.09 seconds
Smarter Mutex: 0.04 seconds
```

It's *really* fast! You're only accessing the mutex once per thread, and doing the calculations in a local variable inside the thread.

There's a lesson here: do as much as you can with local data, and only synchronize when you have data to share. You can't *always* avoid locking the mutex---but if you don't need it, don't use it.