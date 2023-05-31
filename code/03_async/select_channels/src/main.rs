use tokio::sync::{mpsc, broadcast};

async fn receiver(mut rx: mpsc::Receiver<u32>, mut broadcast_rx: broadcast::Receiver<u32>) {
    loop {
        tokio::select! {
            Some(n) = rx.recv() => println!("Received message {n} on the mpsc channel"),
            Ok(n) = broadcast_rx.recv() => println!("Received message {n} on the broadcast channel"),
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<u32>(1);
    let (broadcast_tx, broadcast_rx) = broadcast::channel::<u32>(1);

    tokio::spawn(receiver(rx, broadcast_rx));

   for count in 0 .. 10 {
        if count % 2 == 0 {
            tx.send(count).await.unwrap();
        } else {
            broadcast_tx.send(count).unwrap();
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
