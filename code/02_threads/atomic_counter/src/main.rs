use std::sync::atomic::AtomicU32;

// It's not mutable anymore!
static COUNTER: AtomicU32 = AtomicU32::new(0);

fn main() {
    let mut handles = Vec::new();
    for _ in 0..1_000 {
        let handle = std::thread::spawn(|| {
            for _ in 0..10_000 {
                COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("{}", COUNTER.load(std::sync::atomic::Ordering::Relaxed));
}
