use futures::executor::block_on;
use futures::join;
use futures::future::join_all;

async fn do_work() {
    // Start two tasks at once
    join!(say_hello(), say_goodbye());

    // Return data from an await
    println!("2 * 5 = {}", double(5).await);

    // Return data from a join
    let (a, b) = join!(double(2), double(4));
    println!("2*2 = {a}, 2*4 = {b}");

    // Join a LOT of data
    let futures = vec![double(1), double(2), double(3), double(4)];
    let results = join_all(futures).await;
    println!("Results: {results:?}");

    not_async();
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

fn not_async() {
    println!("This is not async");
}

fn main() {
    block_on(do_work());
}