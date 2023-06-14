# Let's Use Less Bandwidth

We're using about 130 bytes per message. That's not a lot, but if WidgetCorp were to sell 1,000,000 devices---it adds up pretty fast. Any reduction in transmission bandwidth is a good thing, especially if it also reduces overall load.

We're currently serializing our payload data into JSON. JSON is a great format for human-readable data, but it's not the most efficient format for machine-to-machine communication. The number "1.2345" will use 6 bytes (one per ASCII entry), when it could comfortable fit into a single 32-bit (4 byte) float. We're also using a lot of space for field names, which are repeated for every message.

Open the `shared` project.

> The code for this is in `code/05_server/shared_v3`.

The Serde crate is format-agnostic. In `Cargo.toml` we're currently using `serde_json`:

```toml
[dependencies]
crc32fast = "1.3.2"
serde = { version = "1.0.164", features = ["derive"] }
serde_json = "1.0.96"
```

Let's use Bincode, Mozilla's tight format.

We'll add a dependency:

```bash
cargo add bincode -F i128
```

And *remove* `serde_json`. Our resulting `Cargo.toml` looks like this:

```toml
[dependencies]
bincode = { version = "1.3.3", features = ["i128"] }
crc32fast = "1.3.2"
serde = { version = "1.0.164", features = ["derive"] }

```

Now we can simply replace the `to_string` using JSON with `to_vec` using CBOR:

```rust
pub fn encode_v1(command: &CollectorCommandV1) -> Vec<u8> {
    let payload_bytes = bincode::serialize(command).unwrap();
    //let json = serde_json::to_string(&command).unwrap();
    //let json_bytes = json.as_bytes();
    let crc = crc32fast::hash(&payload_bytes);
    let payload_size = payload_bytes.len() as u32;
    let timestamp = unix_now();

    // Encode into bytes
    let mut result = Vec::with_capacity(140);
    result.extend_from_slice(&MAGIC_NUMBER.to_be_bytes());
    result.extend_from_slice(&VERSION_NUMBER.to_be_bytes());
    result.extend_from_slice(&timestamp.to_be_bytes());
    result.extend_from_slice(&payload_size.to_be_bytes());
    result.extend_from_slice(&payload_bytes);
    result.extend_from_slice(&crc.to_be_bytes());
    result
}
```

The decoder needs one change:

```rust
(timestamp, bincode::deserialize(payload).unwrap())
```

Run `cargo test` and your round-trip still works.

## Let's Give it a Go

Now let's open up the client and server. They need to be recompiled (the repo code points to v3 now), and will run with no other changes! That's one of the joys of the crate system and format-agnostic Serde: it's pretty easy to change formats.

Let's add a temporary `println` to see how large our payload is now:

In `main.rs` on the collector:

```rust
while let Ok(command) = rx.recv() {
    let encoded = shared_v3::encode_v1(&command);
    println!("Encoded: {} bytes", encoded.len());
```

We're down to 56 bytes! That's a 57% reduction in size. If you're using a provider that makes you pay for bandwidth, you just saved a bunch of money.

## How Are We Doing on Size?

We've added UUID, random number generation and moved to `bincode`. Are we still small? Our binary is 529,408 bytes (517 kb). That's still pretty small. Resource manager tells me that we're up to 11 mb of committed data.

However... leaving it running without the server running shows this gradually ticking upwards as we enqueue more data!