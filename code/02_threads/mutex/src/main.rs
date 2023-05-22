use std::sync::Mutex;

static NUMBERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

fn main() {
    let mut thread_handles = Vec::new();
    for _ in 0..10 {
        let handle = std::thread::spawn(|| {
            for _ in 0..10_000 {
                let mut lock = NUMBERS.lock().unwrap();
                lock.push(1);
            }
        });
        thread_handles.push(handle);
    }

    thread_handles.into_iter().for_each(|h| h.join().unwrap());

    let lock = NUMBERS.lock().unwrap();
    println!("Numbers length: {}", lock.len());
}
