async fn hello() {
    println!("Hello from async");
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    hello().await;
}
