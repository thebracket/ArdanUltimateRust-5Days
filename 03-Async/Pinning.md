# Pinning, Boxing and Recursion

## Terminology

"Pinning" means storing an async data type in a fixed position in system memory, so that it can safely be used by multiple async tasks. Let's start with a an example that doesn't work.

"Boxing" means putting a variable onto the *heap* inside a smart pointer (that cleans it up when it drops out of scope).

Recursion means calling a function from within itself. This is a common way to implement a Fibonacci Sequence, for example.

## Async Recursion

> The code for this is in `03_async/recursion`.

Wikipedia tells me that a Fibonacci Sequence is characterized by the fact that every number after the first two is the sum of the two preceding ones. Let's build a simple synchronous Fibonacci function:

```rust
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n-1) + fibonacci(n-2)
    }
}

#[tokio::main]
async fn main() {
    println!("fibonacci(10) = {}", fibonacci(10));
}
```

This works fine. What if we want to make it asynchronous?

```rust
async fn async_fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n-1).await + fibonacci(n-2).await
    }
}
```

That looks great, right? Oh dear - it won't compile! The error message is:

```
error[E0733]: recursion in an `async fn` requires boxing
 --> 03_async\recursion\src\main.rs:9:37
  |
9 | async fn async_fibonacci(n: u32) -> u32 {
  |                                     ^^^ recursive `async fn`
  |
  = note: a recursive `async fn` must be rewritten to return a boxed `dyn Future`
  = note: consider using the `async_recursion` crate: https://crates.io/crates/async_recursion
```

If you go digging into `async_recursion`, you'll learn that it is a wrapper around many types:

```
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a, Global>>;
```

The `BoxFuture` type itself comes from the `futures` crate, which we've used plenty of times. Let's add it:

```
cargo add futures
```

You can then use `BoxFuture` for recursion. The syntax isn't pleasant:

```rust
/// https://rust-lang.github.io/async-book/07_workarounds/04_recursion.html
use futures::future::{BoxFuture, FutureExt};
fn async_fibonacci(n: u64) -> BoxFuture<'static, u64> {
    async move {
        match n {
            0 => 0,
            1 => 1,
            _ => async_fibonacci(n - 1).await + async_fibonacci(n - 2).await,
        }
    }.boxed()
}

#[tokio::main]
async fn main() {
    println!("fibonacci(10) = {}", async_fibonacci(10).await);
}
```

I included a link to the Rust "async book" which includes this workaround. I would literally have never figured that out on my own!

### Let's do it the easy way

Fortunately, the `async_recursion` crate can save you from getting a terrible headache when you need to use recursion in async land. Let's add it:

```
cargo add async_recursion
```

Then we can use it, and it provides procedural macros to remove the pain:

```rust
use async_recursion::async_recursion;

#[async_recursion]
async fn async_fibonacci_easier(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => async_fibonacci_easier(n - 1).await + async_fibonacci_easier(n - 2).await,
    }
}
```

## Pinning

> The code for this is in `03_async/pinning`.

Let's start with something that works:

```rust
#[tokio::main]
async fn main() {
    let future = async {
        println!("Hello, world!");
    };
    future.await;
}
```

Now suppose that you are receiving your future from another source, and you receive a *reference* to it (maybe because you have futures returning futures!)

This doesn't work:

```rust
#[tokio::main]
async fn main() {
    let future = async {
        println!("Hello, world!");
    };
    (&mut future).await;
}
```

The error message is: `[async block@03_async\pinning\src\main.rs:3:18: 5:6]` cannot be unpinned
consider using `Box::pin`

So let's pin it using Tokio's `pin!` macro:

```rust
#[tokio::main]
async fn main() {
    let future = async {
        println!("Hello, world!");
    };
    tokio::pin!(future);
    (&mut future).await;
}
```

That does work! But WHY?

* Pinning the future guarantees that it won't move in memory for as long as the `pin` exists.
* Without a pin, the future could move in memory, and that would be bad---and violate Rust's memory safety guarantees.
* With a pin, the future can't move---so all is well.

# Another Variant

Suppose you really like the old "Command Pattern", in which data results in function pointers determining what to do next. It can be handy in some instances, such as a game server or making your very own Turing machine.

You can return a `Future` from a function, but what if you want to return different futures (with the same function signature)? The syntax is a little scary, but this is one to keep around for reference:

```rust
async fn one() {
    println!("One");
}

async fn two() {
    println!("Two");
}

async fn call_one(n: u32) -> Pin<Box<dyn Future<Output = ()>>> {
    match n {
        1 => Box::pin(one()),
        2 => Box::pin(two()),
        _ => panic!("invalid"),
    }
}

async fn main() {
    let boxed_future = call_one(1).await;
    boxed_future.await;
}
```

So why does this work? Working outwards from the middle:
* `Future<Output = ()>` means a future that returns nothing.
* `dyn Future` means a future that implements the `Future` trait, rather than a concrete signature. It's *dynamic*.
* `Box` wraps the type in a smart pointer (a pointer that will automatically clean up after itself). You *have* to use a pointer here, because the contents is dynamic---the compiler can't know ahead of time how large the future will be, so it can't allocate it on the stack.
* `Pin` means that the future is pinned in memory, so it can't move around. This is necessary because the future is stored on the heap, and the compiler can't know ahead of time how large it will be.

The function `Box::pin` is a special type of pointer initialization that not only makes a smart pointer, but pins it in memory so that it won't move.

Phew! That's a lot of stuff to remember. Fortunately, you can just copy and paste this code when you need it.

