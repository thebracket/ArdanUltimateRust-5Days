# Hello Async/Await

There's a simple rule to remember in async/await land:

* Async functions *can* execute non-async functions (and do all the time).
* Non-async functions *cannot* execute async functions, except with the help of an executor.

## Futures

> The code for this is in `code/03_async/hello_async_futures`.

Let's build a really simple example:

```rust
async fn say_hello() {
    println!("Hello, world!");
}

fn main() {
    let x = say_hello();
}
```

This doesn't do anything. Even though `say_hello()` *looks* like it's calling the "say_hello" function---it's not. The type hint in Visual Studio Code gives it away: `impl Future<Output = ()>`. This is a *future*. It represents a task that hasn't been executed yet. You can pass it to an executor to run it, but you can't run it directly.

So let's add an executor. We'll start by using one of the simplest executors out there---a proof of concept more than a real executor. It's called `block_on` and it's in the `futures` crate. We'll start by adding the crate:

```bash
cargo add futures
```

Now, we'll use the simplest bridge between synchronous and asynchronous code: `block_on`. This is a function that takes a future and runs it to completion. It's not a real executor, but it's good enough for our purposes:

```rust
use futures::executor::block_on;

async fn say_hello() {
    println!("Hello, world!");
}

fn main() {
    let _x = say_hello();
    block_on(say_hello());
}
```

The `futures` crate has implemented a simple *executor*, which provides the ability to "block" on an async function. It runs the function---and any async functions it calls---to completion.

Let's add a second async function, and call it from the first:

```rust
use futures::executor::block_on;

async fn say_hello() {
    println!("Hello, world!");
    second_fn().await;
}

async fn second_fn() {
    println!("Second function");
}

fn main() {
    let _x = say_hello();
    block_on(say_hello());
}
```

Notice that once you are *inside* an `async` context, it's easier to call the next async function. You just call it and add `.await` at the end. No need to block again. The "await" keyword tells the executor to run an async task (returned as a future) and wait until it's done.

This is the building block of async/await. You can call async functions from other async functions, and the executor will run them to completion.

## What's Actually Happening Here?

When you call `block_on`, the `futures` crate sets up an execution context. It's basically a list of tasks. The first async function is added to the list and runs until it awaits. Then it moves to the back of the list, and a new task is added to the list. Once the second function completes, it is removed from the task list---and execution returns to the first task. Once there are no more tasks, this simple executor exits.

In other words, you have *cooperative* multitasking. You can await as many things as you want. This particular executor doesn't implement a threaded task pool (unless you ask for it)---it's a single threaded job.

## Other ways to run tasks

> The code for this is in `code/03_async/hello_async_spawn_futures`.

*Join* is used to launch multiple async functions at once:

```rust
use futures::executor::block_on;
use futures::join;

async fn do_work() {
    // Start two tasks at once
    join!(say_hello(), say_goodbye());
}

async fn say_hello() {
    println!("Hello world!");
}

async fn say_goodbye() {
    println!("Goodbye world!");
}

fn main() {
    block_on(do_work());
}
```

You can return data from async functions:

```rust
use futures::executor::block_on;
use futures::join;

async fn do_work() {
    // Start two tasks at once
    join!(say_hello(), say_goodbye());

    // Return data from an await
    println!("2 * 5 = {}", double(5).await);

    // Return data from a join
    let (a, b) = join!(double(2), double(4));
    println!("2*2 = {a}, 2*4 = {b}");
}

async fn say_hello() {
    println!("Hello world!");
}

async fn say_goodbye() {
    println!("Goodbye world!");
}

async fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    block_on(do_work());
}
```

You can even join a vector of futures:

```rust
let futures = vec![double(1), double(2), double(3), double(4)];
let results = join_all(futures).await;
println!("Results: {results:?}");
```

## Async functions can call non-async functions

Add a function:

```rust
fn not_async() {
    println!("This is not async");
}
```

You can call it in `do_work` just like a normal function: `not_async();`.

That's a *lot* of the basics of using `async/await` in Rust. Everything we've done is single-threaded, and isn't super-useful---but with the basics in place, we can start to build more complex applications.