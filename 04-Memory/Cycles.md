# Cycles and Difficult Linked Lists

Linked Lists are one of the first things you learn in many programming languages. They're a simple data structure that can be used to build more complex data structures, and they're a good way to learn about pointers and memory management. Rust provides one: `std::collections::LinkedList`. It's a doubly-linked list, which means that each node in the list has a pointer to the next node and a pointer to the previous node. This makes it easy to insert and remove nodes from the middle of the list, but it also means that the list can form a cycle: the last node in the list can point to the first node in the list.

Actually creating a Linked List in Rust is notoriously tricky. [Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/) is a great resource for learning about the difficulties.

> It should be noted that a linked list is almost never the right data structure to use in modern computer architectures. Chasing pointers for each record FAR outweighs the cache benefits of contiguous data, and move/realloc is *fast* nowadays. Informal testing shows that `Vec` outperforms an actual linked list almost every time.

## A Simple Linked List

> See `code/04_mem/linked_list` for code.

Let's build a simple linked list type structure:

```rust
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

    println!("head: {head:?}");
}

```

You've used all of the elements here: `RefCell` gives you the ability to modify a cell. `Rc` gives you a reference count. `Option` gives you the ability to have a null value. `RefCell` and `Rc` are both wrappers around a value, so you can't have a `RefCell<Option<Rc<Node>>>`---you need to wrap the `Option` in an `Rc` to get `Rc<RefCell<Option<Rc<Node>>>>`. (You can also use `Box` instead of `Rc` here, but `Rc` is more flexible.)

When you run the example, you get a nicely formatted layout of your list:

```
head: Node { value: 2, next: RefCell { value: Some(Node { value: 1, next: RefCell { value: None } }) } }
```

You also see that the list is disposed of:

```
Dropping node with value 2
Dropping node with value 1
```

Now let's add one line of chaos code:

```rust
// Let's break things
*tail.next.borrow_mut() = Some(head.clone());
```

You can no longer print the structure. You get an infinite cycle of `Refcell`... until a stack overflow occurs. Your reference has broken the debug printer! So we comment out the `println`, and it gets worse. There's no output. The reference cycle has prevented Rust from ever decrementing the reference count to zero, so the nodes are never dropped. You've created a memory leak.

One approach is to not do that. Another is to use a *weak reference*.

## Breaking the Cycle

A weak reference is a pointer to an `Rc` or `Arc` reference-counted type that *does not increase the reference count*. It can also become invalid, so to actually use it---you need to call `upgrade` on it. Let's add a weak reference to our linked list:

> The code for this is in `code/04_mem/linked_list_weak`.

```rust
use std::{cell::RefCell, rc::{Rc, Weak}};

#[derive(Debug)]
struct Node {
    value: i32,
    next: RefCell<NextNode>,
}

#[derive(Debug)]
enum NextNode {
    None,
    Strong(Rc<Node>),
    Weak(Weak<Node>),
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping node with value {}", self.value);
    }
}

fn main() {
    let tail = Rc::new(Node {
        value: 1,
        next: RefCell::new(NextNode::None),
    });
    let head = Rc::new(Node {
        value: 2,
        next: RefCell::new(NextNode::Strong(tail.clone())),
    });

    *tail.next.borrow_mut() = NextNode::Weak(Rc::downgrade(&head));

    println!("head: {head:?}");
}
```

We've created an enum for possible links named `NextNode` just to make it easy to store different linkages. If you don't have a strong `Rc` somewhere, the value will never be saved. Then we use `Rc::downgrade` to create a *weak* pointer for the circular list. Bingo - you can print and all values are dropped.