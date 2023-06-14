# Bi-Directional Communication

> See the `code/05_server/collector_v3` and `code/05_server/server_v3` projects.

## Acknowledging Data

We're currently relying on everything working once the TCP data is delivered. We could add some resilience by specifically ACKing packets when they have been received.

Let's start by defining a return data type:

In `shared`, add the following enum:

```rust
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CollectorResponseV1 {
    Ack(),
}
```

We'll also create an encode/decode function pair for it:

```rust
pub fn encode_response_v1(command: CollectorResponseV1) -> Vec<u8> {
    bincode::serialize(&command).unwrap()
}

pub fn decode_response_v1(bytes: &[u8]) -> CollectorResponseV1 {
    bincode::deserialize(bytes).unwrap()
}
```

And make a unit test to check it:

```rust
#[test]
fn test_encode_decode_response() {
    let response = CollectorResponseV1::Ack(123);
    let encoded = encode_response_v1(response.clone());
    let decoded = decode_response_v1(&encoded);
    assert_eq!(decoded, response);
}
```

With that in place, let's open the server code. Go to the `collector.rs` file, and we want to *reply* to the newly arrived packet. We're already checking that an error didn't occur, so we're part way there:

```rust
if result.is_err() {
    println!("Error inserting data into the database: {result:?}");
} else {
    let ack = CollectorResponseV1::Ack;
    let bytes = encode_response_v1(ack);
    socket.write_all(&bytes).await.unwrap();
}
```

So now after we receive a packet, we send a reply acknowledging receipt.

The collector will also have to be updated to receive the reply. We'll add a new function to the `collector.rs` file:

```rust
pub fn send_queue(queue: &mut VecDeque<Vec<u8>>) -> Result<(), CollectorError> {
    // Connect
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS)
        .map_err(|_| CollectorError::UnableToConnect)?;

    // Send every queue item
    let mut buf = vec![0u8; 512];
    while let Some(command) = queue.pop_front() {
        if stream.write_all(&command).is_err() {
            queue.push_front(command);
            return Err(CollectorError::UnableToSendData);
        }
        let bytes_read = stream.read(&mut buf).map_err(|_| CollectorError::UnableToReceiveData)?;
        if bytes_read == 0 {
            queue.push_front(command);
            return Err(CollectorError::UnableToReceiveData);
        }
        let ack = decode_response_v1(&buf[0..bytes_read]);
        if ack != CollectorResponseV1::Ack {
            queue.push_front(command);
            return Err(CollectorError::UnableToReceiveData);
        } else {
            // Comment this out for production
            println!("Ack received");
        }
    }

    Ok(())
}
```

Let's add the error (to `errors.rs`) we used:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CollectorError {
    #[error("Unable to connect to the server")]
    UnableToConnect,
    #[error("Unable to send data to the server")]
    UnableToSendData,
    #[error("Unable to receive data")]
    UnableToReceiveData,
}
```

Lastly, we'll decorate `main.rs` to show any errors:

```rust
// Listen for commands to send
let mut send_queue = VecDeque::with_capacity(120);
while let Ok(command) = rx.recv() {
    let encoded = shared_v3::encode_v1(&command);
    //println!("Encoded: {} bytes", encoded.len());
    send_queue.push_back(encoded);
    let result = sender::send_queue(&mut send_queue);
    if result.is_err() {
        println!("{result:?}");
    }
}
```

Run the server and the collector. You should see a steady stream of "ack received". So now you are only enqueueing data when the server *actually* processed it, as well as receiving it.