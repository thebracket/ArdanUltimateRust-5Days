use std::{sync::atomic::AtomicU32, time::Instant};

static ATOMIC_COUNTER: AtomicU32 = AtomicU32::new(0);
static mut UNSAFE_COUNTER: i32 = 0;

fn unsafe_and_inaccurate() {
    let mut handles = Vec::new();
    for _ in 0..1_000 {
        let handle = std::thread::spawn(|| {
            for _ in 0..10_000 {
                unsafe {
                    UNSAFE_COUNTER += 1;
                }
            }
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
    unsafe {
        println!("Unsafe (and inaccurate): {UNSAFE_COUNTER}");
    }
}

fn safely_atomic() {
    let mut handles = Vec::new();
    for _ in 0..1_000 {
        let handle = std::thread::spawn(|| {
            for _ in 0..10_000 {
                ATOMIC_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("Atomic: {}", ATOMIC_COUNTER.load(std::sync::atomic::Ordering::Relaxed));
}

fn main() {
    let now = Instant::now();
    unsafe_and_inaccurate();
    let unsafe_elapsed = now.elapsed();

    let now = Instant::now();
    safely_atomic();
    let atomic_elapsed = now.elapsed();

    println!();
    println!("Timing Results:");
    println!("Unsafe: {:?} seconds", unsafe_elapsed.as_secs_f32());
    println!("Atomic: {:?} seconds", atomic_elapsed.as_secs_f32());
}
