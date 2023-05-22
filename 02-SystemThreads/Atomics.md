# Sharing Data with Atomics

As we saw at the end of the first class, global variables in Rust aren't always the easiest to work with. Rust turns almost every data race into a compile-time error, which is great, but it also means that we have to be careful about how we share data between threads.

> See the `data_race` code in the `code/02_threads` folder.

## Concurrently Shooting Yourself in the Foot

*Why doesn't this work?*

```rust
static mut COUNTER: i32 = 0;

fn main() {
    let mut handles = Vec::new();
    for _ in 0..1_000 {
        let handle = std::thread::spawn(|| {
            for _ in 0..1_1000 {
                unsafe {
                    COUNTER += 1;
                }
            }
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
    unsafe {
        println!("{COUNTER}");
    }
}
```

We've used `unsafe` to turn off Rust's borrow-checking. **Don't do this**. This is intended to show you why you don't want to do this!

1,000 threads are each iterating 1,000 times, adding one to `COUNTER`. Easy enough, right? Every time you run this program, you get a different result---and none of them are equal to 1,000 * 10,000!

This is happening because adding one to a variable isn't an `atomic` operation (an `atomic` operation is guaranteed to complete before it can be interrupted). It's actually three operations:

1. Read the value of the variable
2. Add one to the value
3. Write the value back to the variable

Threads can be interrupted at any time, and sometimes they are interrupted before all of these steps have been completed. This is called a `data race`, and Rust will not allow you to do this in safe code.

## Atomics to the Rescue

> See the `atomic_counter` code in the `code/02_threads` folder.

```rust
use std::sync::atomic::AtomicU32;

// It's not mutable anymore!
static COUNTER: AtomicU32 = AtomicU32::new(0);

fn main() {
    let mut handles = Vec::new();
    for _ in 0..1_000 {
        let handle = std::thread::spawn(|| {
            for _ in 0..10_000 {
                COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("{}", COUNTER.load(std::sync::atomic::Ordering::Relaxed));
}
```

* Notice that `COUNTER` is no longer mutable. Atomics are an *interior mutability* type (we'll look at these more later). The actual atomic itself can't be changed, but functions implemented by the type can be called---and because they handle synchronization themselves, they can change the internal value.
* `fetch_add` is an atomic operation. It's guaranteed to complete before it can be interrupted.
* `Ordering` is a hint to the compiler about how to handle synchronization.
* `Relaxed` means that the compiler can reorder operations, but it can't reorder operations from different threads. This is the fastest option, but it's not always the best option.
* `load` is also an atomic operation. It's guaranteed to complete before it can be interrupted.

Every time you run this program, you get the same result---and the result is correct, because the atomic type has prevented a data race from occurring.

## Atomic Ordering

`Relaxed` is the fastest option, and allows the compiler to re-order code---while ensuring that the operation is still atomic. This is the default option, and it's the one you should use unless you have a reason not to.

See [the documentation](https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html) for a full description of all of your ordering options. If you are performing a task that absolutely requires that each thread operates in perfect order, use `SeqCst` (sequential consistency). This is the slowest option, but it guarantees that all operations will be performed in the order they were called.

## So How Fast are Atomics?

> See the `atomic_counter_timed` code in the `code/02_threads` folder.

```rust
use std::{sync::atomic::AtomicU32, time::Instant};

static ATOMIC_COUNTER: AtomicU32 = AtomicU32::new(0);
static mut UNSAFE_COUNTER: i32 = 0;

fn unsafe_and_inaccurate() {
    let mut handles = Vec::new();
    for _ in 0..1_000 {
        let handle = std::thread::spawn(|| {
            for _ in 0..10_000 {
                unsafe {
                    UNSAFE_COUNTER += 1;
                }
            }
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
    unsafe {
        println!("Unsafe (and inaccurate): {UNSAFE_COUNTER}");
    }
}

fn safely_atomic() {
    let mut handles = Vec::new();
    for _ in 0..1_000 {
        let handle = std::thread::spawn(|| {
            for _ in 0..10_000 {
                ATOMIC_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("Atomic: {}", ATOMIC_COUNTER.load(std::sync::atomic::Ordering::Relaxed));
}

fn main() {
    let now = Instant::now();
    unsafe_and_inaccurate();
    let unsafe_elapsed = now.elapsed();

    let now = Instant::now();
    safely_atomic();
    let atomic_elapsed = now.elapsed();

    println!();
    println!("Timing Results:");
    println!("Unsafe: {:?} seconds", unsafe_elapsed.as_secs_f32());
    println!("Atomic: {:?} seconds", atomic_elapsed.as_secs_f32());
}
```

Running this in `debug` mode shows that even with range-checking and other debug features turned on, the unsafe version is slightly faster. It also gives the wrong answer, so it's not very useful:

```
Unsafe (and inaccurate): 814665
Atomic: 10000000

Timing Results:
Unsafe: 0.1302608 seconds
Atomic: 0.20318009 seconds
```

Release mode (`cargo run --release`) optimizes the code:

```
Unsafe (and inaccurate): 10000000
Atomic: 10000000

Timing Results:
Unsafe: 0.0321137 seconds
Atomic: 0.1295855 seconds
```

Both versions are faster. The `unsafe` version is *really* fast, and happens to get the correct answer. The `atomic` version is slower, but it's still pretty fast---and it's guaranteed to get the correct answer. If you were doing more calculations, the `release` version with unsafe code would become progressively more inaccurate over time.

The moral of this story: `release` mode can hide bugs. Don't use `unsafe` and pray that your bug is masked. Use `atomic` and be sure that your code is correct.