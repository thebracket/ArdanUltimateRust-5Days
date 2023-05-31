# Tokio Futures

Most of the async/await code you've created works exactly the same in Tokio. However, Tokio tends to "take over" a bit and has its own syntax for a few things. It also offers a few options that you won't necessarily find elsewhere.

> See the `03_async/tokio_await` code.

## Await

The `.await` system is completely unchanged:

```rust
async fn hello() {
    println!("Hello from async");
}

#[tokio::main]
async fn main() {
    hello().await;
}
```

## Joining

Tokio provides a `join!` macro that you can use just like you did with `futures::join!`:

```rust
let result = tokio::join!(double(2), double(3));
println!("{result:?}");
```

If you have a vector of futures, Tokio does *not* provide a `join_all` macro! You can import the `futures` crate and use that one (it will still run inside Tokio---it uses whatever the current executor is).

Add the `futures` crate as well with `cargo add futures`. Then:

```rust
// You can still use futures join_all
let futures = vec![double(2), double(3)];
let result = futures::future::join_all(futures).await;
println!("{result:?}");
```

Alternatively, you can use Tokio's native `JoinSet` type. The code looks like this:

```rust
// Using Tokio JoinSet
let mut set = JoinSet::new();
for i in 0..10 {
    set.spawn(double(i));
}
while let Some(res) = set.join_next().await {
    println!("{res:?}");
}
```

Notice that every `res` returned is a `Result`. Even though your function didn't return a result, it wrapped everything in `Ok`.

> You can also drop the join set and it will automatically cancel any pending futures for you.

I personally tend ot use the `futures` version unless I really need the extra control.

## Spawning

What if you want to start an async task, and not wait for it to complete? Tokio provides the `spawn!` macro for this purpose. It's just like a thread spawn, but it adds as async task to the task pool (which may or may not be on another actual thread). Let's try this example:

```rust
async fn ticker() {
    for i in 0..10 {
        println!("tick {i}");
    }
}

#[tokio::main]
async fn main() {
    tokio::spawn(ticker());
    hello().await;
}
```

(We're keeping the `hello()` function from before). Run it, and notice that the answers appear in a different order---the threading system is distributing work. It's also not guaranteed that `ticker` will complete, because the program isn't waiting for it to finish. Let's try another approach:

```rust
#[tokio::main]
async fn main() {
    let _ = tokio::join!(
        tokio::spawn(hello()), 
        tokio::spawn(ticker()),
    );
    println!("Finished");
}
```

Now you are sending `hello` and `ticker` into separate tasks, and waiting for all of them. The program will wait for everything to finish.

## Threading

Notice that the previous program gives different results each time. That's because you are in the multi-threaded executor. let's try the single-threaded mode:

```rust
#[tokio::main(flavor = "current_thread")]
```

You always get the same result:

```
Hello from async
(8, 12)
[8, 12]
tick 0
tick 1
tick 2
tick 3
tick 4
tick 5
tick 6
tick 7
tick 8
tick 9
Ok(0)
Ok(4)
Ok(8)
Ok(12)
Ok(16)
Ok(20)
Ok(24)
Ok(28)
Ok(32)
Ok(36)
Finished
```

What's going on here?

The `hello` task starts first. Printing doesn't *yield*---there's no await. So it always runs that. Calling `double` does yield, so it goes into the task pool. It runs the double commands, which also allows the next joined task to arrive in the task pool. The `ticker` runs. Note that `ticker` doesn't `await` anywhere, so it always runs as one large blob. Then the `JoinSet` runs, which is yielding for each call.

That's a pitfall of async programming. If `ticker` were doing something complicated, in a single-threaded environment---or a really busy task pool with threads---you are effectively "locking up" the other tasks until it completes.

Fortunately, you can *also* explicitly "yield" control:

```rust
async fn ticker() {
    for i in 0..10 {
        println!("tick {i}");
        tokio::task::yield_now().await;
    }
}
```

`yield_now` is telling Tokio that you are done for now, allow other tasks to run. Just like a thread, when your task resumes---it's stack will be restored, and it will continue as before. This is a good way to make sure that you don't lock up the task pool. It also slows your computation down!

Run it now - notice that the ticks run once and then last. `yield_now` moves the task to the back of the queue, so it will run again when it's ready.

`yield_now` is useful if you *must* do something CPU intensive in your async task. If possible, send your big task over to a thread. We'll look at that in a bit.