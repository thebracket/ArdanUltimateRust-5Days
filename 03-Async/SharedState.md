# Shared State (Tokio)

You may remember dealing with global variables [last week](../02-SystemThreads/Mutexes.md). There are async versions of the same primitives, but it's not always clear which you should use when.

## Atomic Variables

Atomic variables are *completely untouched* by async. So everything you learned [in last week's class on atomics](../02-SystemThreads/Atomics.md) applies to async land, too. They are still high-performance, great ways to share data when you can.

## Mutexes

> The code for this is in `03_async/async_mutex`.

(Add `once_cell` to your project with `cargo add`)

You can still use a system mutex in async land:


```rust
use std::sync::Mutex;

static COUNTER: Mutex<u32> = Mutex::new(0);

async fn increment() {
    let mut counter = COUNTER.lock().unwrap();
    *counter += 1;
}

#[tokio::main]
async fn main() {
    tokio::join!(increment(), increment(), increment());
    println!("COUNTER = {}", *COUNTER.lock().unwrap());
}
```

If you don't have much contention, this is still a high-performance option. The Tokio documentation even recommends it in many cases. BUT - and there's always a but - it has some issues in `async` land. There are two issues:

* If the mutex is contested, you can block a whole thread while you wait.
* You can't pass a standard-library mutex between async tasks.

Let's look at the second problem:

```rust
use std::sync::Mutex;

static COUNTER: Mutex<u32> = Mutex::new(0);

async fn add_one(n: u32) -> u32 {
    n + 1
}

async fn increment() {
    let mut counter = COUNTER.lock().unwrap();
    *counter = add_one(*counter).await;
}

#[tokio::main]
async fn main() {
    tokio::join!(increment(), increment(), increment());
    println!("COUNTER = {}", *COUNTER.lock().unwrap());
}
```

Notice that this compiles and runs. Clippy gives a very serious sounding warning, though:

```bash
cargo clippy
```

```
warning: this `MutexGuard` is held across an `await` point
  --> 03_async\async_mutex\src\main.rs:10:9
   |
10 |     let mut counter = COUNTER.lock().unwrap();
   |         ^^^^^^^^^^^
   |
   = help: consider using an async-aware `Mutex` type or ensuring the `MutexGuard` is dropped before calling await
note: these are all the `await` points this lock is held through
  --> 03_async\async_mutex\src\main.rs:10:5
   |
10 | /     let mut counter = COUNTER.lock().unwrap();
11 | |     *counter = add_one(*counter).await;
12 | | }
   | |_^
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#await_holding_lock
   = note: `#[warn(clippy::await_holding_lock)]` on by default

warning: `async_mutex` (bin "async_mutex") generated 1 warning
```

What this means is that the regular `MutexGuard` type you get from calling `lock()` assumes that you are in a threaded world. It is unaware of a serious danger: the mutex remains locked when you `await`, and can cause a deadlock by accessing `COUNTER` from another *task*.

So Tokio provides an `async` version that you can use instead when you need to use Mutexes inside an async context. It works very similarly, but you have to `await` your locks:

```rust
//use std::sync::Mutex;
use tokio::sync::Mutex;
use once_cell::sync::Lazy;

//static COUNTER: Mutex<u32> = Mutex::new(0);
static COUNTER: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));

async fn add_one(n: u32) -> u32 {
    n + 1
}

async fn increment() {
    //let mut counter = COUNTER.lock().unwrap();
    let mut counter = COUNTER.lock().await;
    *counter = add_one(*counter).await;
}

#[tokio::main]
async fn main() {
    tokio::join!(increment(), increment(), increment());
    //println!("COUNTER = {}", *COUNTER.lock().unwrap());
    println!("COUNTER = {}", *COUNTER.lock().await);
}
```

## Aside: What's with the Lazy?

This is a great opportunity to expand a bit on what we talked about before with initializing statics.

This is valid:
```rust
use std::sync::Mutex;
static COUNTER: Mutex<u32> = Mutex::new(0)
```

This isn't:

```rust
use tokio::sync::Mutex;
static COUNTER: Mutex<u32> = Mutex::new(0);
```

Why would that be? `static` functions can only be initialized with a function that is marked as `const`.

You can provide any `const` function for initialization:

```rust
use std::sync::Mutex;
static CONST_MUTEX : Mutex<i32> = Mutex::new(new_mutex());

const fn new_mutex() -> i32 {
    5 * 12
}
```

But you *can't* use a function that isn't constant. Tokio's mutex `new` isn't constant, so you can't use it directly. But you can use `Lazy` (from `once_cell` and very soon the standard library) to add a layer of indirection that calls a non-const function through a closure:

```rust
static CONST_MUTEX : Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("Hello".to_string()));
```

## RwLock

Read/Write locks have exactly the same change. You can use the `tokio` version just like a standard library `rwlock`, but you have to `await` your `read()` and `write()` calls.