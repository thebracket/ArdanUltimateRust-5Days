use std::{sync::{atomic::AtomicU32, Mutex}, time::Instant};

static ATOMIC_COUNTER: AtomicU32 = AtomicU32::new(0);
static mut UNSAFE_COUNTER: u32 = 0;
static MUTEX_COUNTER: Mutex<u32> = Mutex::new(0);
static MUTEX_COUNTER2: Mutex<u32> = Mutex::new(0);

const N_THREADS: usize = 1_000;
const N_ITERATIONS: usize = 10_000;

fn unsafe_and_inaccurate() {
    let mut handles = Vec::new();
    for _ in 0..N_THREADS {
        let handle = std::thread::spawn(|| {
            for _ in 0..N_ITERATIONS {
                unsafe {
                    UNSAFE_COUNTER += 1;
                }
            }
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
    unsafe {
        println!("Unsafe (and inaccurate)  : {UNSAFE_COUNTER}");
    }
}

fn safely_atomic() {
    let mut handles = Vec::new();
    for _ in 0..N_THREADS {
        let handle = std::thread::spawn(|| {
            for _ in 0..N_ITERATIONS {
                ATOMIC_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("Atomic                   : {}", ATOMIC_COUNTER.load(std::sync::atomic::Ordering::Relaxed));
}

fn mutex_locked() {
    let mut handles = Vec::new();
    for _ in 0..N_THREADS {
        let handle = std::thread::spawn(|| {
            for _ in 0..N_ITERATIONS {
                *MUTEX_COUNTER.lock().unwrap() += 1;
            }
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("Mutex                    : {}", *MUTEX_COUNTER.lock().unwrap());
}

fn smarter_mutex_locked() {
    let mut handles = Vec::new();
    for _ in 0..N_THREADS {
        let handle = std::thread::spawn(|| {
            let mut n = 0;
            for _ in 0..N_ITERATIONS {
                n += 1;
            }
            *MUTEX_COUNTER2.lock().unwrap() += n;
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("Smarter Mutex            : {}", *MUTEX_COUNTER2.lock().unwrap());
}

fn main() {
    let now = Instant::now();
    unsafe_and_inaccurate();
    let unsafe_elapsed = now.elapsed();

    let now = Instant::now();
    safely_atomic();
    let atomic_elapsed = now.elapsed();

    let now = Instant::now();
    mutex_locked();
    let mutex_elapsed = now.elapsed();

    let now = Instant::now();
    smarter_mutex_locked();
    let smarter_mutex_elapsed = now.elapsed();

    println!();
    println!("Timing Results:");
    println!("Unsafe:        {:.2} seconds", unsafe_elapsed.as_secs_f32());
    println!("Atomic:        {:.2} seconds", atomic_elapsed.as_secs_f32());
    println!("Mutex:         {:.2} seconds", mutex_elapsed.as_secs_f32());
    println!("Smarter Mutex: {:.2} seconds", smarter_mutex_elapsed.as_secs_f32());
}
