use std::sync::mpsc;

type Job = Box<dyn FnOnce() + Send + 'static>;

fn hi_there() {
    println!("Hi there!");
}

fn main() {
    let (tx, rx) = mpsc::channel::<Job>();
    let handle = std::thread::spawn(move || {
        while let Ok(job) = rx.recv() {
            job();
        }
    });

    let job = || println!("Hello from the thread!");
    let job2 = || {
        for i in 0..10 {
            println!("i = {i}");
        }
    };
    tx.send(Box::new(job)).unwrap();
    tx.send(Box::new(job2)).unwrap();
    tx.send(Box::new(hi_there)).unwrap();
    tx.send(Box::new(|| println!("Inline!"))).unwrap();
    handle.join().unwrap();
}
