fn main() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    // We're using a scope to ensure that we wait for everything to finish
    pool.scope(|scope| {
        for n in 0..4 {
            scope.spawn(move |_scope | {
                println!("Hello from top-level {n}");
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();
                
                pool.scope(|scope| {
                    for inner_n in 0.. 4 {
                        scope.spawn(move |_scope| {
                            println!("Hello from inner {inner_n} (part of {n})");
                        });
                    }
                });
                
                println!("Goodbye from top-level {n}");
            });
        }
    });
}
