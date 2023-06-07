# Packing, Re-ordering and Mangling

## Packing

Let's take a simple program and guess what it does:

```rust
struct OneByte {
    a: u8,
}

struct TwoBytes {
    a: u16,
}

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
```

The result may surprise you:

```
1
2
4
4
```

Rust has *aligned* your 24-bit structure to a 32-bit boundary in memory. It's allowed to do this with the default "packing" scheme. In general, this speeds up execution---it's easier for the CPU to access memory on word-size boundaries. It does mean we're wasting 1 byte of memory, though. If you are working with a lot of data, this can add up---more realistically, if you are parsing bit data to and from a file or network, you may be surprised when your data is not the size you expect.

You can tell Rust that you care about exact byte alignment by adding the decoration:

```rust
#[repr(packed)]
struct ThreeBytes {
    a: u16,
    b: u8,
}
```

Running the program again, we get the more expected "1, 2, 3, 4".

## Re-Ordering

On top of changing your structure size, Rust reserves the right to rearrange your structure! This is called "re-ordering". Rust is allowed to do this because it doesn't change the semantics of your program---you access fields by name. But if you are doing binary serialization, you may be surprised when your data is not in the order you expect.

You can tell Rust that you need your structure to be in the order you defined it by adding the decoration:

```rust
#[repr(C)]
```

> You can combine these decorations, e.g. `#[repr(packed, C)]`.

## Mangling

Rust has a concept of "mangling" names. This is a way of encoding the type information into the name of the function. The linker will use these "internal" names to resolve symbols. This is a way of avoiding name collisions. It also means that you can't call a Rust function from C without some extra work.

You can tell Rust to not mangle a name by adding the decoration:

```rust
#[no_mangle]
```

If you are working with other languages (via the foreign function interface), you may need to use this decoration. Otherwise, the other language will not be able to find your function.

So on the boundaries of your program, where you are dealing with binary data and/or other languages you may need to remember `#[repr(C)])]` and `#[no_mangle]`. You may need `#[repr(packed)]`---but most other languages also pack. Be aware of packing for serialization!