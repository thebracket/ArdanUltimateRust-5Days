# Setting the Collector ID

> See `code/05_server/collector_v2` for the code

Always having an id of 0 identifying our data-collector isn't all that useful. We need to be able to identify which collector is sending us data. We'll use a UUID. Imagine that as part of the setup process it is assigned a UUID, and that UUID is stored in a file on the collector.

You need to add a crate:

```bash
cargo add uuid -F v4 -F fast-rng
```

The `v4` and `fast-rng` options allow easy generation of UUIDs.

Now let's add to `main.rs` to obtain a UUID from a file, and create one if it isn't present.

```rust
fn get_uuid() -> u128 {
    let path = std::path::Path::new("uuid");
    if path.exists() {
        let contents = std::fs::read_to_string(path).unwrap();
        contents.parse::<u128>().unwrap()
    } else {
        let uuid = uuid::Uuid::new_v4().as_u128();
        std::fs::write(path, uuid.to_string()).unwrap();
        uuid
    }
}
```

There's no need to send the UUID as a giant string, so we're treating it as a `u128`. Now modify `main()` to use it:

```rust
fn main() {
    let uuid = get_uuid();
    let (tx, rx) = std::sync::mpsc::channel::<CollectorCommandV1>();

    // Start the collector thread
    let _collector_thread = std::thread::spawn(move || {
        data_collector::collect_data(tx, uuid);
    });
```

And change the `collect_data()` function to use it:

```rust
pub fn collect_data(tx: Sender<CollectorCommandV1>, collector_id: u128) {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_memory();
    sys.refresh_cpu();
    std::thread::sleep(std::time::Duration::from_secs_f32(1.0));
    loop {
        let now = Instant::now();

        // Refresh the stored data
        sys.refresh_memory();
        sys.refresh_cpu();

        // Get new values
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let num_cpus = sys.cpus().len();
        let total_cpu_usage = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>();
        let average_cpu_usage = total_cpu_usage / num_cpus as f32;

        // Submit
        let send_result = tx.send(CollectorCommandV1::SubmitData {
            collector_id,
            total_memory,
            used_memory,
            average_cpu_usage,
        });
```

And now your collector is uniquely identified---and keeps its ID on restart.
