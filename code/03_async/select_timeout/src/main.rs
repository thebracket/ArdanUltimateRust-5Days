use std::time::Duration;
use tokio::time::sleep;

async fn do_work() {
    // Pretend to do some work that takes longer than expected
    sleep(Duration::from_secs(2)).await;
}

async fn timeout(seconds: f32) {
    // Wait for the specified number of seconds
    sleep(Duration::from_secs_f32(seconds)).await;
}

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = do_work() => println!("do_work() completed first"),
        _ = timeout(1.0) => println!("timeout() completed first"),
    }
}
