# Reference Counting

Rust doesn't have a garbage collector. With RAII/Drop, a *lot* of the time you won't notice that it's not there. Sometimes, you *do* need to share data between multiple owners - and only "drop" the data when *all* owners are done with it. This is where reference counting comes in.

> Reference counting is a *form* of garbage collection. It's the weakest form---generational or even mark/sweep can give better results, which is why languages such as Go and Java use them.

## What is an `Rc`

The Rust `Rc` type is a lot like a `Box`, but it's designed to be shared.

> The code for this is in `code/04_mem/refcount`.

Let's take our `Droppable` type from the last exercise, and use a reference counter to share it:

```rust
use std::rc::Rc;

#[derive(Debug)]
struct Droppable(i32);

impl Droppable {
    fn new(n: i32) -> Self {
        println!("Constructing {n}");
        Self(n)
    }
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn move_me(x: Rc<Droppable>) {
    println!("Moved {}", x.0);
}

fn main() {
    let my_shared = Rc::new(Droppable::new(1));
    {
        let _x = my_shared.clone();
        let _y = my_shared.clone();
        let _z = my_shared.clone();
    }
    move_me(my_shared.clone());

    println!("{my_shared:?}");
    println!("Application exit");
}
```

The output is:

```
Constructing 1
Moved 1
Droppable(1)
Application exit
Dropping 1
```

So we've only made one shareable variable, and then cloned references to it. When the last reference is dropped, the `Drop` implementation fires. It's also very high performance: it adds one variable to the stored type (the reference count), which may even be merged into structure padding if there is any.

## How Does This Work with Threads?

`Rc` isn't a `Send` type, so sending it to threads doesn't work. However, `Arc` is a thread-safe reference counter. It's a bit slower than `Rc`, but it's safe to send to threads.

> The code for this is in `code/04_mem/atomicrefcount`.

The same code---with some added threading---works across threads:

```rust
use std::sync::Arc;

#[derive(Debug)]
struct Droppable(i32);

impl Droppable {
    fn new(n: i32) -> Self {
        println!("Constructing {n}");
        Self(n)
    }
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn move_me(x: Arc<Droppable>) {
    println!("Moved {}", x.0);
}

fn main() {
    let my_shared = Arc::new(Droppable::new(1));
    {
        let _x = my_shared.clone();
        let _y = my_shared.clone();
        let _z = my_shared.clone();
    }
    move_me(my_shared.clone());

    let mut threads = Vec::new();
    for _ in 0.. 10 {
        let my_shared = my_shared.clone();
        threads.push(std::thread::spawn(move || {
            move_me(my_shared)
        }));
    }
    for t in threads {
        t.join().unwrap();
    }

    println!("{my_shared:?}");
    println!("Application exit");
}
```

The output shows that the single variable remains shared, even though its accessed by the main thread and 10 more.

## How About Mutability?

It gets a little more complicated when you want to be able to *change* the variable. `Arc` is "Send" safe---you can send it between threads. But it doesn't provide any synchronization, so it can't be used alone to safely change a variable across threads.

### Option 1: "External" Arc<Mutex>

> The code for this is in `code/04_mem/external_arc_mutex`.

The first option is to include a `Mutex` (or other lock) inside your `Arc`. You're still sharing data---including the Mutex itself. You can use the `Mutex` exactly as you have in global/static variables:

```rust
use std::sync::{Arc, Mutex};

struct SharedData(String);

fn main() {
    let my_shared = Arc::new(Mutex::new(SharedData("Hello".to_string())));
    let mut threads = Vec::new();
    for i in 0..10 {
        let my_shared = my_shared.clone();
        threads.push(std::thread::spawn(move || {
            let mut data = my_shared.lock().unwrap();
            data.0.push_str(&format!(" {i}"));
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
    let data = my_shared.lock().unwrap();
    println!("{}", data.0);
}
```

### Option 2: "Internal" Arc<Mutex>

> The code for this is in `code/04_mem/internal_arc_mutex`.

The problem with the external `Arc<Mutex<Data>>` pattern is that it starts to become a very long type---especially if you are sharing a type that also has its own embedded type data. The solution is to use a `Mutex` inside the type itself. This is called *interior mutability*.

It's almost the same---but we've pushed the `Mutex` inside the type:

```rust
use std::sync::{Arc, Mutex};

struct SharedData{
    data: Mutex<String>,
}

impl SharedData {
    fn new(s: &str) -> Self {
        Self {
            data: Mutex::new(s.to_string()),
        }
    }
}

fn main() {
    let my_shared = Arc::new(SharedData::new("Hello"));
    let mut threads = Vec::new();
    for i in 0..10 {
        let my_shared = my_shared.clone();
        threads.push(std::thread::spawn(move || {
            let mut data = my_shared.data.lock().unwrap();
            data.push_str(&format!(" {i}"));
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
    let data = my_shared.data.lock().unwrap();
    println!("{data}");
}
```

This works because `Sync` and `Send` are automatically transitive properties in Rust.

* A type can be `Sync` if all of its members are `Sync`, allowing it to be shared---accessed and possibly modified---between threads.
* A type can be `Send` if all of its members are `Send`, allowing it to be sent between threads.

`Arc` is `Send`, providing a safe wrapper to its contents. `Mutex` (and other locks) are `Sync`, allowing them to be shared between threads. So `Arc<Mutex<T>>` is both `Send` and `Sync`, allowing it to be shared and sent between threads.

These properties automatically apply to types. So if your type contains a `Sync` type (and nothing that isn't `Sync`), it is automatically `Sync`. If your type contains a `Send` type (and nothing that isn't `Send`), it is automatically `Send`. You can achieve `Send` by wrapping in a type such as `Arc`---or building that into the type as well.

## What if I Don't Need Threads?

`Mutex` and `RwLock` are quite heavyweight, and they're not needed if you don't need to share between threads. You can use `RefCell` instead. It's a lot like `Mutex`---but it's not thread-safe. It's also a lot faster.

> CAUTION! Using `RefCell` in a context in which multiple threads *might* access it is dangerous. It's not thread-safe, and it will panic if you try to access it from multiple threads. It's only safe if you *know* that it will only be accessed from a single thread.

> The code for this is in `code/04_mem/refcell`.

`RefCell` marks an area of memory as being run-time borrowed. You can borrow it mutably or immutably, and the borrow checker will enforce the general rules---but you bypass them by manually calling `borrow` and `borrow_mut`. This is "safe" because the borrow checker will panic if you try to borrow mutably while there is an immutable borrow, or if you try to borrow mutably while there is already a mutable borrow.

```rust
use std::{cell::RefCell, sync::Arc};

struct MyData {
    data: RefCell<String>
}

impl MyData {
    fn new() -> Self {
        Self {
            data: RefCell::new("Hello".to_string())
        }
    }
}

fn move_data(data: Arc<MyData>) {
    let mut data = data.data.borrow_mut();
    data.push_str(" World");
}

fn main() {
    let shared_data = Arc::new(MyData::new());
    move_data(shared_data.clone());
    let data = shared_data.data.borrow();
    println!("{data}");
}
```

It's very fast to borrow in this fashion---there's no synchronization at all. But don't try it with threads!