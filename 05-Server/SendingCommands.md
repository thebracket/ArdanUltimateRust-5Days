# Sending Commands

Now that we have a bi-directional communications interface, we could make our widget do more than just be a pure data collector. We could send commands to it, and have it do things. That potentially makes the widget much more useful!

Let's add a second type of command to our shared data structures:

```rust
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CollectorCommandV1 {
    SubmitData {
        collector_id: u128,
        total_memory: u64,
        used_memory: u64,
        average_cpu_usage: f32,
    },
    RequestWork(u128), // Contains the collector id
}
```

The idea is that after submitting data, the collector can ask "is there any work for me to do?" The server can then reply with "no, there isn't" or "yes, there is". Let's extend the response, also:

```rust
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CollectorResponseV1 {
    Ack,
    NoWork,
    Task(TaskType)
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TaskType {
    Shutdown,
}
```

We'll also need to update our server to handle the new command. Open `collector.rs` and adjust the `match` statement:

```rust
match received_data {
            (_timestamp, CollectorCommandV1::RequestWork(_collector_id)) => {
                // Do something
            }
            (timestamp, CollectorCommandV1::SubmitData { collector_id, total_memory, used_memory, average_cpu_usage }) => {
```

For now, let's always reply that there's nothing to do:

```rust
(_timestamp, CollectorCommandV1::RequestWork(_collector_id)) => {
    let no_work = CollectorResponseV1::NoWork;
    let bytes = encode_response_v1(no_work);
    socket.write_all(&bytes).await.unwrap();
}
```

And we go over to the collector, and submit the work request:

Change the `send_queue` function to require the collector's ID:

```rust
pub fn send_queue(queue: &mut VecDeque<Vec<u8>>, collector_id: u128) -> Result<(), CollectorError> {
```

And add a new section to the bottom:

```rust
// Ask for work
let bytes = shared_v3::encode_v1(&shared_v3::CollectorCommandV1::RequestWork(collector_id));
if stream.write_all(&bytes).is_err() {
    return Err(CollectorError::UnableToSendData);
}
let bytes_read = stream.read(&mut buf).map_err(|_| CollectorError::UnableToReceiveData)?;
if bytes_read == 0 {
    return Err(CollectorError::UnableToReceiveData);
}
let work = decode_response_v1(&buf[0..bytes_read]);
match work {
    CollectorResponseV1::NoWork => {}
    CollectorResponseV1::Task(task) => {
        println!("Task received: {task:?}");
    }
    _ => {}
}

Ok(())
```

Lastly, you'll need to send the `uuid` variable in `main.rs`:

```rust
let result = sender::send_queue(&mut send_queue, uuid);
```

Run the collector and server now, just to check that it still works.

## Shutting Down Collectors

Let's provide a system to let web users issue shutdown commands to collectors.

Let's add `once_cell` to the server: `cargo add once_cell`.

We'll start by making a `commands.rs` file in the server's `src` directory (and adding `mod commands;` to `main.rs`):

```rust
use once_cell::sync::Lazy;
use shared_v3::TaskType;
use std::sync::Mutex;
use std::collections::HashMap;

static COMMANDS: Lazy<Mutex<HashMap<u128, TaskType>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn add_command(collector_id: u128, command: TaskType) {
    let mut commands = COMMANDS.lock().unwrap();
    commands.insert(collector_id, command);
}

pub fn get_commands(collector_id: u128) -> Option<TaskType> {
    let mut commands = COMMANDS.lock().unwrap();
    commands.remove(&collector_id)
}
```

Now in `api.rs`, we'll add an API end-point to submit the command:

```rust
pub async fn shutdown_collector(uuid: Path<String>) {
    let uuid = uuid::Uuid::parse_str(uuid.as_str()).unwrap();
    let uuid = uuid.as_u128();
    add_command(uuid, shared_v3::TaskType::Shutdown);
}
```

And add a route for it in `main.rs`:

```rust
.route("/api/collector/:uuid/shutdown", get(api::shutdown_collector))
```

Finally, the collector server needs to check for commands:

```rust
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
```

Now you can run the collector and server. Visit [http://localhost:3000/](http://localhost:3000/) and copy a UUID.

Then paste that into your browser as:
http://localhost:3000/api/collector/UUID/shutdown

The collector should show a shutdown command.

Congratulations---you now have a bidirectional widget system. You're still only using 567,296 (554k) of disk space on the collector.