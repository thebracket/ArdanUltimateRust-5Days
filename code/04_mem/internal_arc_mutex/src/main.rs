use std::sync::{Arc, Mutex};

struct SharedData{
    data: Mutex<String>,
}

impl SharedData {
    fn new(s: &str) -> Self {
        Self {
            data: Mutex::new(s.to_string()),
        }
    }
}

fn main() {
    let my_shared = Arc::new(SharedData::new("Hello"));
    let mut threads = Vec::new();
    for i in 0..10 {
        let my_shared = my_shared.clone();
        threads.push(std::thread::spawn(move || {
            let mut data = my_shared.data.lock().unwrap();
            data.push_str(&format!(" {i}"));
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
    let data = my_shared.data.lock().unwrap();
    println!("{data}");
}
