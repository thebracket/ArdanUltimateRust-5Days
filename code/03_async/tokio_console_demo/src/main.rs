use std::net::SocketAddr;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    spawn,
};

#[tracing::instrument(name="echo", fields(address=%address))]
async fn echo_stream(mut socket: TcpStream, address: SocketAddr) {
    tracing::info!("New Connection from {:?}", address);
    let mut buf = vec![0; 1024];
    loop {
        let n = socket
            .read(&mut buf)
            .await
            .expect("failed to read data from socket");
        tracing::info!("Received {} bytes from {address:?}", n);

        if n == 0 {
            tracing::warn!("No bytes received from {address:?}. Closing connection.");
            return;
        }

        socket
            .write_all(&buf[0..n])
            .await
            .expect("failed to write data to socket");
    }
}

#[tracing::instrument(name = "listener")]
async fn listen() -> anyhow::Result<()> {
    // Listen for connections
    let listener = TcpListener::bind("127.0.0.1:8123").await?;
    tracing::info!("Listening on port 8123");

    loop {
        let (socket, address) = listener.accept().await?;
        spawn(echo_stream(socket, address));
    }
}

#[tracing::instrument(name = "client")]
async fn client() -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8123").await?;
    tracing::info!("Connected to the server!");

    for _ in 0..10 {
        // Send "Hello World"
        stream.write_all(b"Hello World!").await?;

        // Read the response
        let mut buf = vec![0; 1024];
        let bytes_read = stream.read(&mut buf).await?;
        tracing::info!("Response: {}", String::from_utf8_lossy(&buf[..bytes_read]));
        tokio::time::sleep(std::time::Duration::from_secs_f32(0.1)).await;
    }

    Ok(())
}

#[tracing::instrument(name = "spawner")]
async fn client_spawner() -> anyhow::Result<()> {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs_f32(1.0));
    loop {
        interval.tick().await;
        spawn(client());
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the Tokio Console subscription
    console_subscriber::init();

    // Start the server
    spawn(listen());

    // Start the periodic client
    client_spawner().await?;

    Ok(())
}
