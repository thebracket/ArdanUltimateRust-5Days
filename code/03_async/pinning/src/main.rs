use std::{future::Future, pin::Pin};

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

#[tokio::main]
async fn main() {
    let future = async {
        println!("Hello, world!");
    };
    tokio::pin!(future);
    (&mut future).await;

    let boxed_future = call_one(1).await;
    boxed_future.await;
}