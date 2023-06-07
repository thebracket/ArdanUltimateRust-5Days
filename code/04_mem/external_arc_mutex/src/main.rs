use std::sync::{Arc, Mutex};

struct SharedData(String);

fn main() {
    let my_shared = Arc::new(Mutex::new(SharedData("Hello".to_string())));
    let mut threads = Vec::new();
    for i in 0..10 {
        let my_shared = my_shared.clone();
        threads.push(std::thread::spawn(move || {
            let mut data = my_shared.lock().unwrap();
            data.0.push_str(&format!(" {i}"));
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
    let data = my_shared.lock().unwrap();
    println!("{}", data.0);
}
