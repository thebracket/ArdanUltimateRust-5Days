use std::{cell::RefCell, sync::Arc};

struct MyData {
    data: RefCell<String>
}

impl MyData {
    fn new() -> Self {
        Self {
            data: RefCell::new("Hello".to_string())
        }
    }
}

fn move_data(data: Arc<MyData>) {
    let mut data = data.data.borrow_mut();
    data.push_str(" World");
}

fn main() {
    let shared_data = Arc::new(MyData::new());
    move_data(shared_data.clone());
    let data = shared_data.data.borrow();
    println!("{data}");
}
