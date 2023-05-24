fn main() {
    // Let's explicitly size our thread pool
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.scope(|scope| {
        scope.spawn_broadcast(|_scope, broadcast_context| {
            // You can use scope just like the parent scope
            println!("Hello from broadcast thread {}", broadcast_context.index());
        });
    });
}
