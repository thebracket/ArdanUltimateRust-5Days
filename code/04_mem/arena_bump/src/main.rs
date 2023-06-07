use bumpalo::Bump;
use bumpalo::collections::String;
use bumpalo::collections::Vec;

struct MyData {
    a: i32,
}

fn main() {
    let arena = Bump::new();
    arena.set_allocation_limit(Some(8192)); // Limit the size of the arena to 8 KiB
    let x = arena.alloc(MyData { a: 123 });

    // With collections enabled
    let mut my_string = String::from_str_in("Hello, world!", &arena);
    let mut vec = Vec::new_in(&arena);
    vec.push(12);
}
