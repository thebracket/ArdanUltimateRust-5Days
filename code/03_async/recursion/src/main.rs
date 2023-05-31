#[allow(dead_code)]
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n-1) + fibonacci(n-2)
    }
}

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

use async_recursion::async_recursion;
#[async_recursion]
async fn async_fibonacci_easier(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => async_fibonacci_easier(n - 1).await + async_fibonacci_easier(n - 2).await,
    }
}

#[tokio::main]
async fn main() {
    println!("fibonacci(10) = {}", async_fibonacci(10).await);
    println!("fibonacci(10) = {}", async_fibonacci_easier(10).await);
}