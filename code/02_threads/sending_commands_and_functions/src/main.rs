use std::sync::mpsc;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Command {
    Run(Job),
    Quit,
}

fn main() {
    let (tx, rx) = mpsc::channel::<Command>();
    let handle = std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::Run(job) => job(),
                Command::Quit => break,
            }
        }
    });

    let job = || println!("Hello from the thread!");
    tx.send(Command::Run(Box::new(job))).unwrap();
    tx.send(Command::Quit).unwrap();
    handle.join().unwrap();
}
