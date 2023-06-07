struct Droppable(i32);

impl Droppable {
    fn new(n: i32) -> Self {
        println!("Constructing {n}");
        Self(n)
    }
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn move_me(x: Droppable) {
    println!("Moved {}", x.0);
}

struct HasDrop {
    id: i32,
    d: Droppable,
}

impl HasDrop {
    fn new(id: i32) -> Self {
        Self {
            id,
            d: Droppable::new(id + 100),
        }
    }
}

fn main() {
    let _x = Droppable::new(1);
    let _y = Droppable::new(2);
    let _z = Droppable::new(3);

    {
        let _scoped = Droppable::new(4);
        println!("Ending scope");
    }
    println!("Ended scope");

    let a = Droppable::new(5);
    move_me(a);
    println!("Function returned");

    // Transitive drops
    let _b = HasDrop::new(600);

    println!("Exiting main");
}
