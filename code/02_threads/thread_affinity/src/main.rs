fn main() {
    let core_ids = core_affinity::get_core_ids().unwrap();

    let handles = core_ids.into_iter().map(|id| {
        std::thread::spawn(move || {
            // Pin this thread to a single CPU core.
            let success = core_affinity::set_for_current(id);
            if success {
                println!("Hello from a thread on core {id:?}");
            } else {
                println!("Setting thread affinity for core {id:?} failed");
            }
        })
    }).collect::<Vec<_>>();
    
    for handle in handles.into_iter() {
        handle.join().unwrap();
    }
}
