# From Bytes to Types

You often need to convert a binary format---bytes---info a Rust type (and vice versa). You might be reading a file, or parsing a network packet---or interacting with a system written in another programming language. Rust has a few tools to help you with this.

> Note: If you have a specific format in mind, you can use Serde. The "bincode" crate provides a binary format for Serde that is basically a memory dump.

## Saving Bytes to a File

> The code for this is in `code/04_mem/save_bytes`

There are `unsafe` code options to directly transform a structure into an array of bytes, but let's stick with safe code. The `bytemuck` crate is a safe wrapper around `unsafe` code that does this for you.

Add `bytemuck` to your project:

```bash
cargo add bytemuck -F derive
```

```rust
#[repr(C)]
#[derive(bytemuck::Zeroable, bytemuck::Pod, Clone, Copy, Debug)]
struct OurData {
    number: u16,
    tag: [u8; 8],
}

fn main() {
    let some_data = vec![
        OurData {
            number: 1,
            tag: *b"hello   ",
        },
        OurData {
            number: 2,
            tag: *b"world   ",
        }
    ];

    let bytes: &[u8] = bytemuck::cast_slice(&some_data);
    std::fs::write("data.bin", bytes).unwrap();

    // Read the data back
    let bytes = std::fs::read("data.bin").unwrap();
    let data: &[OurData] = bytemuck::cast_slice(&bytes);

    // Debug print the data to show the round-trip worked
    println!("{data:?}");
}

```

We define a type with some fixed-sized data in it. We then use bytemuck's `Pod` type (to define Plain Old Data) and `Zeroable` (required by `Pod`---the type can be zeroed in memory). Then we can use `cast_slice` to create a slice of bytes from our data, and write it to a file.

Note that the `bytes` type is a slice of bytes, not a vector. It's a reference to the data in `some_data`, not a copy of it. This is a zero-copy operation. Zero-copy is very, very fast.

## Reading Bytes from a File - and Casting to a Type

Reading the data back is equally straightforward once you have the data:

```rust
// Read the data back
let bytes = std::fs::read("data.bin").unwrap();
let data: &[OurData] = bytemuck::cast_slice(&bytes);

// Debug print the data to show the round-trip worked
println!("{data:?}");
```

`bytes` now contains the concrete data, and `data` is a zero-copied reference to it.

This is a great pattern for fixed-size records in binary data. I've used it to parse netlink data from the Linux kernel in mere nanoseconds.

## Converting Bytes to a String

Strings work differently. In the example above, we used a fixed-size array of bytes with spaces in the gaps. That's very convenient for fixed-size records (and is common in many file formats), but you'd probably rather further transform the data into a string.

```rust
// Print the first record's tag as a string
println!("{}", std::str::from_utf8(&data[0].tag).unwrap());
```

Fortunately, `str` includes a conversion from a slice of bytes to a string. It can fail if the bytes don't align with valid unicode. You may want to `trim` out extra characters---which will make converting back a little trickier.

## Writing a Protocol

> The code for this is in `code/04_mem/save_dynamic_bytes`.

You quite often want to read/write data as a protocol---a stream. This allows you to account for variable sizing.

> If you are in async, Tokio has a "framing" feature to help with this.

Let's write some data to a file:

```rust
use std::{fs::File, io::Write};

struct OurData {
    number: u16,
    tag: String,
}

fn main() {
    let a = OurData {
        number: 12,
        tag: "Hello World".to_string(),
    };

    // Write the record in parts
    let mut file = File::create("bytes.bin").unwrap();

    // Write the number and check that 2 bytes were written
    assert_eq!(file.write(&a.number.to_le_bytes()).unwrap(), 2);

    // Write the string length IN BYTES and check that 8 bytes were written
    let len = a.tag.as_bytes().len();
    assert_eq!(file.write(&(len as u64).to_le_bytes()).unwrap(), 8);

    // Write the string and check that the correct number of bytes were written
    assert_eq!(file.write(a.tag.as_bytes()).unwrap(), len);
}
```

So we're defining some data, and creating a file. Then we write the `number` field as two bytes (specifying endian format). Then we write the length of the string as 8 bytes (a `u64`), and then we write the string itself.

Reading it back is mostly reversing the process:

```rust
///// READ THE DATA BACK
// Read the whole file as bytes.
let bytes = std::fs::read("bytes.bin").unwrap();

// Read the number
let number = u16::from_le_bytes(bytes[0..2].try_into().unwrap());

// Read the string length
let length = u64::from_le_bytes(bytes[2..10].try_into().unwrap());

// Decode the string
let tag = std::str::from_utf8(&bytes[10..(10 + length as usize)]).unwrap();

let a = OurData {
    number,
    tag: tag.to_string(),
};
println!("{a:?}");
```

Notice that this isn't zero copy. In an ideal world, we'd do a bit of both. Read descriptors, and use those to cast bytes to types.

You may also want to read the file with a buffered reader, a few bytes at a time if you have memory constraints (or a HUGE file).