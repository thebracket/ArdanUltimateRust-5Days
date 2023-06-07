use std::{rc::Rc, cell::RefCell};

#[derive(Debug)]
struct Node {
    value: i32,
    next: RefCell<Option<Rc<Node>>>
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping node with value {}", self.value);
    }
}

fn main() {
    let tail = Rc::new(Node {
        value: 1,
        next: RefCell::new(None),
    });
    let head = Rc::new(Node {
        value: 2,
        next: RefCell::new(Some(tail.clone())),
    });

    // Let's break things
    //*tail.next.borrow_mut() = Some(head.clone());

    //println!("head: {head:?}");
}
