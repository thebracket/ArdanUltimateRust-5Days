# The Data Collector: Version 0.1

We have a few objectives for the data collector:

* Data is collected once per second, aiming for precise collision.
* Data is transmitted to the server as it is collected.
    * However, transmission slowdowns must not slow down collection.
* We're aiming for lean & mean---small, running on a small device. (We're going to allow ourselves to say that the device has full Rust support, and is running Linux.)

> The initial server is in `code/05_server/collector_v1`.

Create a new workspace project, `collector`. You'll need to add two crates:

`cargo add sysinfo -F apple-app-store`. The feature `apple-app-store` is there for convenience; it prevents us from using calls that won't work on Apple devices.

We also need to add a reference to our `shared` crate:

```toml
[package]
name = "collector_v1"
version = "0.1.0"
edition = "2021"

[dependencies]
shared_v1 = { path = "../shared_v1" }
sysinfo = { version = "0.29.2", features = ["apple-app-store"] }
```

## Collecting Data

Let's start by writing a quick function that gathers data and sends it via a channel. Create a file in the `src` directory, named `data_collector.rs`. In `main.rs` add the line `mod data_collector;` to the top of the file.

```rust
use shared_v1::CollectorCommandV1;
use sysinfo::{SystemExt, CpuExt};
use std::{time::Instant, sync::mpsc::Sender};

pub fn collect_data(tx: Sender<CollectorCommandV1>) {
    // Initialize the sysinfo system
    let mut sys = sysinfo::System::new_all();

    // Perform a single refresh and pause. `sysinfo` gathers data via deltas,
    // and the first reading is usually useless.
    sys.refresh_memory();
    sys.refresh_cpu();
    std::thread::sleep(std::time::Duration::from_secs_f32(1.0));

    // Run forever
    loop {
        // Note when we're starting
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
            collector_id: 0,
            total_memory,
            used_memory,
            average_cpu_usage,
        });
        if let Err(e) = send_result {
            println!("Error sending data: {e:?}");
        }

        // Wait for the next cycle
        let elapsed_seconds = now.elapsed().as_secs_f32();
        if elapsed_seconds < 1.0 {
            std::thread::sleep(std::time::Duration::from_secs_f32(1.0 - elapsed_seconds));
        } else {
            // Warning: we're running behind!
            std::thread::sleep(std::time::Duration::from_secs_f32(1.0));
        }
    }
}
```

This is a pretty simple function. We're using the `sysinfo` crate to gather data about the system, and then we're sending it to the server via a channel. We're also doing a bit of work to ensure that we're sending data once per second, even if the data collection takes longer than that.

## Sending data

Create another file in `src`, named `sender.rs`. Add the line `mod sender;` to the top of `main.rs`.

```rust
use std::io::Write;
use shared_v1::{CollectorCommandV1, DATA_COLLECTOR_ADDRESS};

pub fn send_command(command: CollectorCommandV1) {
    let bytes = shared_v1::encode_v1(command);
    println!("Encoded {} bytes", bytes.len());
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS).unwrap();
    stream.write_all(&bytes).unwrap();
}
```

This is a *very* simple function at this point. We're using the `encode_v1` function we created in the shared library, and sending the encoded packet via TCP. We're not using `async`---we're aiming for lean and mean.

## The `main` function

Let's fill out the `main` function:

```rust
use shared_v1::CollectorCommandV1;
mod data_collector;
mod sender;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<CollectorCommandV1>();

    // Start the collector thread
    let _collector_thread = std::thread::spawn(move || {
        data_collector::collect_data(tx);
    });

    // Listen for commands to send
    while let Ok(command) = rx.recv() {
        sender::send_command(command);
    }
}
```

The benefit of using a separate thread for collection is that it is scheduled independently, and won't be slowed down by the network. The main thread is just listening for commands to send, and calling the `send_command` function.

There's no error handling yet, but we'll get to that. It's always a good idea to get a minimally functional system to validate your basic design before you make *nice* functionality!

## How Are We Doing on Size?

* We're transmitting 132 bytes per packet, which is a good start. We can improve that.
* Our binary, compiled in `release` mode is 516,608 bytes (504 kb). That's not bad, but we can do better. It'll fit just fine on something like a Raspberry Pi Zero already.
* We'll have to build a basic server to receive the data before we can start measuring CPU and RAM usage.

Let's build a very minimal data-collection server.