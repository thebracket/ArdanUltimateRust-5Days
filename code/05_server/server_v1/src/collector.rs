use std::net::SocketAddr;
use shared_v1::{DATA_COLLECTOR_ADDRESS, decode_v1};
use tokio::{net::{TcpListener, TcpStream}, io::AsyncReadExt};

pub async fn data_collector() -> anyhow::Result<()> {
    // Listen for TCP connections on the data collector address
    let listener = TcpListener::bind(DATA_COLLECTOR_ADDRESS).await?;

    // Loop forever, accepting connections
    loop {
        // Wait for a new connection
        let (socket, address) = listener.accept().await?;
        tokio::spawn(new_connection(socket, address));
    }
}

async fn new_connection(mut socket: TcpStream, address: SocketAddr) {
    println!("New connection from {address:?}");
    let mut buf = vec![0u8; 1024];
    loop {
        let n = socket
            .read(&mut buf)
            .await
            .expect("failed to read data from socket");

        if n == 0 {
            println!("No data received - connection closed");
            return;
        }

        println!("Received {n} bytes");
        let received_data = decode_v1(&buf[0..n]);
        println!("Received data: {received_data:?}");
    }
}