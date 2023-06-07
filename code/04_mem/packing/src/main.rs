struct OneByte {
    a: u8,
}

struct TwoBytes {
    a: u16,
}

#[repr(packed)]
struct ThreeBytes {
    a: u16,
    b: u8,
}

struct FourBytes {
    a: u32,
}

fn main() {
    println!("{}", std::mem::size_of::<OneByte>());
    println!("{}", std::mem::size_of::<TwoBytes>());
    println!("{}", std::mem::size_of::<ThreeBytes>());
    println!("{}", std::mem::size_of::<FourBytes>());
}
