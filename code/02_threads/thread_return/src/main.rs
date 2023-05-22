fn do_math(i: u32) -> u32 {
    let mut n = i+1;
    for _ in 0 .. 10 {
        n *= 2;
    }
    n
}

fn main() {
    let mut thread_handles = Vec::new();
    for i in 0..10 {
        thread_handles.push(std::thread::spawn(move || {
            do_math(i)
        }));
    }

    for handle in thread_handles {
        println!("Thread returned: {}", handle.join().unwrap());
    }
}
