# Shared Data Structures

It makes sense to share serializable data structures between the data-collector and the data-ingestor server. We'll use the `serde` to provide serialization services.

> Why not REST? You could use REST, and it would be straightforward to build and setup. It's also quite heavy, so you're using more network bandwidth than you need to. When you have a large network of Things sending you data, it really adds up.

## The Submissions Protocol

> The initial server is in `code/05_server/shared_v1`.

Whenever you build a protocol, you should start with a simple version---but also try to include future expansion.

Let's define the initial protocol:

Bytes | Name | Description
--- | --- | ---
0-1 | **Magic Number** | Sending a magic number is a common way to ensure that the data you're receiving is what you expect.
2-3 | **Version Number** | We'll start with version 1. We're going to use two bytes, so we have lots of room for future versions. If we somehow use 65,535 versions, we'll mark a version as indicating that the next bytes are a sub-version!
4-7 | **Timestamp** | We'll use a 32-bit unsigned integer to represent the number of seconds since the Unix epoch. This will give us a range of 1970-01-01 to 2106-02-07.
8-11 | **Payload size** | We'll use a 32-bit unsigned integer to represent the size of the payload.
12+ | **Payload** | We'll start with JSON and move to something more efficient.
End-4 - End | **CRC32** | We'll use a CRC32 checksum to ensure that the data we received is the data we expected. We'll use the `crc32fast` crate to provide this functionality.

> Just think how many meetings it really takes to agree on a protocol!

So the twin concepts expressed in this protocol are:

* *No Surprises*: Having the magic number and version number at the beginning of the packet means that we can immediately reject any packets that don't match our expectations.
* *No Corruption*: Having the CRC32 checksum at the end of the packet means that we can immediately reject any packets that don't match our expectations.
* *Size Included*: You don't have to use `read_to_end`, or scan a stream for an "end" token---you easily keep the TCP conversation going.

## Supporting Crates

In the shared library, use `cargo add` to add:

* `serde` with the `derive` feature.
* `serde_json` for JSON support.
* `crc32fast` for CRC32 support.

## The Payload

Having a "version" number makes easier to change the payload format and content, without having to change the wrapper protocol.

For the payload itself, let's use an enumeration. We're initially just sending the one command type, but it's good to have room for expansion. In the shared library, define a public enum:

```rust
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CollectorCommandV1 {
    SubmitData {
        collector_id: u128, // To be converted from a UUID
        total_memory: u64,
        used_memory: u64,
        average_cpu_usage: f32,
    },
}
```

Notice that we're carefully avoiding `usize`---the size of the `usize` and `isize` types are dependent upon the platform.

Now that we have a payload, let's build an encoding function.

## Encoding Routine

We're going to start out by making a byte vector containing the format we defined above. The protocol says to use UNIX timestamps. Let's add a helper function to do this (you can use the `chrono` crate for really thorough date/time handling, but we're trying to keep things small):

```rust
fn unix_now() -> u32 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs() as u32
}
```

Let's also define the data collector address/port, magic number, version to use as constants:

```rust
pub const DATA_COLLECTOR_ADDRESS: &str = "127.0.0.1:9004";
const MAGIC_NUMBER: u16 = 1234;
const VERSION_NUMBER: u16 = 1;
```

We'll painstakingly add each entry, one at a time:

```rust
pub fn encode_v1(command: CollectorCommandV1) -> Vec<u8> {
    let json = serde_json::to_string(&command).unwrap();
    let json_bytes = json.as_bytes();
    let crc = crc32fast::hash(json_bytes);
    let payload_size = json_bytes.len() as u32;
    let timestamp = unix_now();

    // Encode into bytes
    let mut result = Vec::with_capacity(140);
    result.extend_from_slice(&MAGIC_NUMBER.to_be_bytes());
    result.extend_from_slice(&VERSION_NUMBER.to_be_bytes());
    result.extend_from_slice(&timestamp.to_be_bytes());
    result.extend_from_slice(&payload_size.to_be_bytes());
    result.extend_from_slice(json_bytes);
    result.extend_from_slice(&crc.to_be_bytes());
    result
}
```

I picked "140 bytes" as the initial vector capacity based on a quick test, measuring a packet size.

## Decoding Routine

Decoding is the same process, but in reverse:

```rust
pub fn decode_v1(bytes: &[u8]) -> (u32, CollectorCommandV1) {
    let magic_number = u16::from_be_bytes([bytes[0], bytes[1]]);
    let version_number = u16::from_be_bytes([bytes[2], bytes[3]]);
    let timestamp = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    let payload_size = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
    let payload = &bytes[12..12 + payload_size as usize];
    let crc = u32::from_be_bytes([
        bytes[12 + payload_size as usize],
        bytes[13 + payload_size as usize],
        bytes[14 + payload_size as usize],
        bytes[15 + payload_size as usize],
    ]);

    // Verify the magic number
    assert_eq!(magic_number, MAGIC_NUMBER);

    // Verify the version number
    assert_eq!(version_number, VERSION_NUMBER);

    // Verify the CRC
    let computed_crc = crc32fast::hash(payload);
    assert_eq!(crc, computed_crc);

    // Decode the payload
    (timestamp, serde_json::from_slice(payload).unwrap())
}
```

See how we've added assertions and a CRC check to ensure that we have the same data that was sent?

## Unit Tests

The last thing to do in the shared data library is to create a unit test that round-trip tests the encoding and decoding:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let command = CollectorCommandV1::SubmitData {
            total_memory: 100,
            used_memory: 50,
            average_cpu_usage: 0.5,
        };
        let encoded = encode_v1(command.clone());
        let (timestamp, decoded) = decode_v1(&encoded);
        assert_eq!(decoded, command);
        assert!(timestamp > 0);
    }
}
```

It's *always* a good idea to test encode and decode together. As you add versions, you'll want to add tests for each version.

Now let's start building our basic, initial data collector.