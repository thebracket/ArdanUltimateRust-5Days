fn test() {
    println!("Hello from test thread");
}

fn main() {
    // Let's explicitly size our thread pool
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.join(test, test);
}
