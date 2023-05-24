use std::sync::Mutex;

static DATA: Mutex<u32> = Mutex::new(0);

fn poisoner() {
    let mut lock = DATA.lock().unwrap();
    *lock += 1;
    panic!("And poisoner crashed horribly");
}

fn main() {
    let handle = std::thread::spawn(poisoner);
    println!("Trying to return from the thread:");
    println!("{:?}", handle.join());
    println!("Locking the Mutex after the crash:");
    let lock = DATA.lock();
    println!("{lock:?}");

    // Let's try to save the day by recovering the data from the Mutex
    let recovered_data = lock.unwrap_or_else(|poisoned| {
        println!("Mutex was poisoned, recovering data...");
        poisoned.into_inner()
    });
    println!("Recovered data: {recovered_data:?}");
}
