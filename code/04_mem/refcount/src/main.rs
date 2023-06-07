use std::rc::Rc;

#[derive(Debug)]
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

fn move_me(x: Rc<Droppable>) {
    println!("Moved {}", x.0);
}

fn main() {
    let my_shared = Rc::new(Droppable::new(1));
    {
        let _x = my_shared.clone();
        let _y = my_shared.clone();
        let _z = my_shared.clone();
    }
    move_me(my_shared.clone());

    println!("{my_shared:?}");
    println!("Application exit");
}
