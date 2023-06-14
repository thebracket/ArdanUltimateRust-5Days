use std::net::SocketAddr;
use shared_v3::{DATA_COLLECTOR_ADDRESS, decode_v1, CollectorCommandV1, encode_response_v1, CollectorResponseV1};
use sqlx::{Pool, Sqlite};
use tokio::{net::{TcpListener, TcpStream}, io::{AsyncReadExt, AsyncWriteExt}};
use crate::commands::get_commands;

pub async fn data_collector(cnn: Pool<Sqlite>) -> anyhow::Result<()> {
    // Listen for TCP connections on the data collector address
    let listener = TcpListener::bind(DATA_COLLECTOR_ADDRESS).await?;

    // Loop forever, accepting connections
    loop {
        // Wait for a new connection
        let cnn = cnn.clone();
        let (socket, address) = listener.accept().await?;
        tokio::spawn(new_connection(socket, address, cnn));
    }
}

async fn new_connection(mut socket: TcpStream, address: SocketAddr, cnn: Pool<Sqlite>) {
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

        let received_data = decode_v1(&buf[0..n]);

        match received_data {
            (_timestamp, CollectorCommandV1::RequestWork(collector_id)) => {
                if let Some(commands) = get_commands(collector_id) {
                    let work = CollectorResponseV1::Task(commands);
                    let bytes = encode_response_v1(work);
                    socket.write_all(&bytes).await.unwrap();
                } else {
                    let no_work = CollectorResponseV1::NoWork;
                    let bytes = encode_response_v1(no_work);
                    socket.write_all(&bytes).await.unwrap();
                }
            }
            (timestamp, CollectorCommandV1::SubmitData { collector_id, total_memory, used_memory, average_cpu_usage }) => {
                let collector_id = uuid::Uuid::from_u128(collector_id);
                let collector_id = collector_id.to_string();

                let result = sqlx::query("INSERT INTO timeseries (collector_id, received, total_memory, used_memory, average_cpu) VALUES ($1, $2, $3, $4, $5)")
                    .bind(collector_id)
                    .bind(timestamp)
                    .bind(total_memory as i64)
                    .bind(used_memory as i64)
                    .bind(average_cpu_usage)
                    .execute(&cnn)
                    .await;

                if result.is_err() {
                    println!("Error inserting data into the database: {result:?}");
                } else {
                    let ack = CollectorResponseV1::Ack;
                    let bytes = encode_response_v1(ack);
                    socket.write_all(&bytes).await.unwrap();
                }
            }
        }        
    }
}