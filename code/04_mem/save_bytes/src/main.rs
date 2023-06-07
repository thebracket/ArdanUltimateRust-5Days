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

    // Print the first record's tag as a string
    println!("{}", std::str::from_utf8(&data[0].tag).unwrap());
}
