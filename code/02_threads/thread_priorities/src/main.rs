use std::{sync::atomic::AtomicU32, time::Duration};
use thread_priority::*;

static LOW_COUNT: AtomicU32 = AtomicU32::new(0);
static MEDIUM_COUNT: AtomicU32 = AtomicU32::new(0);

fn low_prio() {
    set_current_thread_priority(ThreadPriority::Min).unwrap();
    loop {
        LOW_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        std::thread::yield_now();
    }
}

fn regular_prio() {
    loop {
        MEDIUM_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        std::thread::yield_now();
    }
}

fn main() {
    std::thread::spawn(low_prio);
    std::thread::spawn(regular_prio);

    std::thread::sleep(Duration::from_secs(10));

    println!("Low    : {:>10}", LOW_COUNT.load(std::sync::atomic::Ordering::Relaxed));
    println!("Medium : {:>10}", MEDIUM_COUNT.load(std::sync::atomic::Ordering::Relaxed));
}
