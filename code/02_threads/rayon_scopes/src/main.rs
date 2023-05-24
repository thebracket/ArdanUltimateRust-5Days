fn main() {
    // Let's explicitly size our thread pool
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.spawn(|| println!("Hello from pool thread"));

    pool.scope(|scope| {
        for n in 0..20 {
            scope.spawn(move |_| {
                println!("Hello from thread {n}");
            });
        }
    });
    println!("Hello from main thread");
}
