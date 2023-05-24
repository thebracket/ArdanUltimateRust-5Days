# Deadlocks

One particularly nasty side-effect of locking your data is a *deadlock*. A Mutex or Read/Write lock blocks until access is obtained. If you try to lock a mutex twice (or obtain write access to an RwLock twice)---even in the same thread---you'll deadlock. The thread will block forever, waiting for itself to release the lock.

Rust can't prevent deadlocks. It provides mechanisms to help avoid them, but you have to use them correctly.

## Deadlocking Example

> The code for this section is in the `deadlocks` directory in the `code/02_threads` directory.

Here's a simple way to completely lock up your program:

```rust
use std::sync::Mutex;

fn main() {
    let my_shared = Mutex::new(0);

    let lock = my_shared.lock().unwrap();
    let lock = my_shared.lock().unwrap();
}
```

The program never stops! My trying to acquire the Mutex twice, the thread deadlocks. It's waiting for itself to release the lock.

## Try Locking

One way to avoid deadlocks is to use `try_lock` instead of `lock`:

```rust
use std::sync::Mutex;

static MY_SHARED : Mutex<u32> = Mutex::new(0);

fn main() {

    if let Ok(_lock) = MY_SHARED.try_lock() {
        println!("I got the lock!");

        // Try again, but this time, the lock is already taken
        if let Ok(_lock) = MY_SHARED.try_lock() {
            println!("I got the lock!");
        } else {
            println!("I couldn't get the lock!");
        }

    } else {
        println!("I couldn't get the lock!");
    }
}
```

The downside here is that `try_lock` doesn't wait. It either succeeds or it fails. If it fails, you can try again later, but you have to be careful not to try too often. If you try too often, you'll end up with a busy loop---hitting the CPU over and over again.

It's better to not write any deadlocks!

## Explicitly Dropping Locks

Locks are "scope guarded". They implement `Drop`, which we'll talk about in the memory management class. If you're used to C++, it's the `RAII`---Resource Acquisition Is Initialization---pattern.

When a lock goes out of scope, it is automatically released. So you don't need to worry about releasing a lock in normal code---it's done for you.

Sometimes, you want to explicitly get rid of a lock. Maybe you're in a function that locks to check something, and later locks again to do something else. You can explicitly drop the lock to release it early. You may want to do this if you're going to be doing a lot of work between locks, and you don't want to hold the lock for that long.

```rust
let lock = MY_SHARED.lock().unwrap();
std::mem::drop(lock);
let lock = MY_SHARED.lock().unwrap();
```

That's ugly, but it works. Calling "drop" invalidates the first lock---you can no longer use that variable.

## Cleaning Up Locks with Scopes

A prettier way to do the same thing is to manually introduce a *scope*. (Note that if you are using a scope, you might want to look at a function for readability!). This is the same as the previous example, but the "drop" is implicit because of the scope:

```rust
// Using a scope to drop a lock
{
    let _lock = MY_SHARED.lock().unwrap();
    println!("I got the lock!");
}
let _lock = MY_SHARED.lock().unwrap();
println!("I got the lock again!");
```

## Mutex Poisoning

> The code for this is in the `mutex_poisoning` directory in the `code/02_threads` directory.

If a thread crashes/panics while holding a lock, the lock becomes *poisoned*. This is a safety feature of Rust: since the thread crashed, you can't be sure that the contents of the lock is safe. So the lock is poisoned, and any attempt to lock it will fail.

Let's have a look:

```rust
use std::sync::Mutex;

static DATA: Mutex<u32> = Mutex::new(0);

fn poisoner() {
    let mut lock = DATA.lock().unwrap();
    *lock += 1;
    panic!("And poisoner crashed horribly");
}

fn main() {
    let handle = std::thread::spawn(poisoner);
    println!("Trying to return from the thread:");
    println!("{:?}", handle.join());
    println!("Locking the Mutex after the crash:");
    let lock = DATA.lock();
    println!("{lock:?}");
}
```

This gives the following output:

```
Trying to return from the thread:
thread '<unnamed>' panicked at 'And poisoner crashed horribly', 02_threads\mutex_poisoning\src\main.rs:8:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Err(Any { .. })
Locking the Mutex after the crash:
Err(PoisonError { .. })
```

So what's happening here?

1. The thread runs and explicitly panics.
2. Panicking in a thread doesn't crash the whole program (unless you've setup a panic handler to do so).
3. Joining a thread that crashed returns an error, telling you that the thread crashed.
4. Since the thread crashed while it had the Mutex locked, the Mutex is poisoned.
5. Any attempt to lock the Mutex will fail.

So the glib answer to avoiding this is "don't crash". Handle errors, be a good program. That's great, but the real world doesn't always work that way. Sometimes, you have to deal with crashes.

## Recovering from Poisoning

In a lot of cases, since you've experienced a failure the correct thing to do is crash! It's often safer to crash than to propagate bad data.

This is one time that `unwrap()` is great---your Mutex is dangerous, so `unwrap` it and crash:

```rust
let lock = DATA.lock().unwrap();
```

Again, in most cases I don't really recommend it - but the `PoisonError` type contains a locked mutex that you can use to obtain the data. **There's absolutely no guaranty that the data is in good shape, depending upon why the thread crashed**. So be careful!

```rust
// Let's try to save the day by recovering the data from the Mutex
let recovered_data = lock.unwrap_or_else(|poisoned| {
    println!("Mutex was poisoned, recovering data...");
    poisoned.into_inner()
});
println!("Recovered data: {recovered_data:?}");
```
