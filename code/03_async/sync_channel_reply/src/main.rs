use std::{time::Duration, sync::mpsc};

enum Command {
    Print(String),
}

#[tokio::main]
async fn main() {
    // Spawn a command thread for "heavy lifting"
    let (tx, rx) = mpsc::channel::<Command>();

    // Spawn a TOKIO Async channel for replies
    let (tx_reply, mut rx_reply) = tokio::sync::mpsc::channel::<String>(10);

    let handle = tokio::runtime::Handle::current();
    std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::Print(s) => {
                    // Make our very own copy of the transmitter
                    let tx_reply = tx_reply.clone();
                    handle.spawn(async move {
                        tx_reply.send(s).await.unwrap();
                    });
                },
            }
        }
    });

    // Launch a Tokio process to receive replies from thread-land
    tokio::spawn(async move {
        while let Some(reply) = rx_reply.recv().await {
            println!("{reply}");
        }
    });

    // Launch the async sender
    let mut counter = 0;
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        tx.send(Command::Print(format!("Hello {counter}"))).unwrap();
        counter += 1;
    }
}
