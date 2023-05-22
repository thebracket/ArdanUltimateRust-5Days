use std::thread;

fn main() {
    const N_THREADS: usize = 8;

    let to_add: Vec<u32> = (0..5000).collect();
    let chunks = to_add.chunks(N_THREADS);
    let sum = thread::scope(|s| {
        let mut thread_handles = Vec::new();

        for chunk in chunks {
            let thread_handle = s.spawn(move || {
                let mut sum = 0;
                for i in chunk {
                    sum += i;
                }
                sum
            });
            thread_handles.push(thread_handle);
        }

        thread_handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .sum::<u32>()
    });
    println!("Sum is {sum}");
}
