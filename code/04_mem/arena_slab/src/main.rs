use slab::Slab;

fn main() {
    let mut slab = Slab::with_capacity(10);
    let hello = slab.insert("hello");
    let world = slab.insert("world");

    assert_eq!(slab[hello], "hello");
    assert_eq!(slab[world], "world");

    slab.remove(hello);
}
