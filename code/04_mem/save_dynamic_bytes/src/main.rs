use std::{fs::File, io::Write};

#[derive(Debug)]
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
}