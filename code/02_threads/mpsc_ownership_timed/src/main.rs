use std::sync::mpsc;

// Not copyable or clone-able
struct MyData {
    start: std::time::Instant,
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    let (tx, rx) = mpsc::channel::<MyData>();

    std::thread::spawn(move || {
        while let Ok(data) = rx.recv() {
            let elapsed = data.start.elapsed();
            println!("--- IN THE THREAD ---");
            println!("Message passed in {} us", elapsed.as_micros());
        }
    });

    loop {
        println!("Enter a string");
        let _ = read_line();
        let data_to_move = MyData {
            start: std::time::Instant::now(),
        };

        tx.send(data_to_move).unwrap();
    }
}
