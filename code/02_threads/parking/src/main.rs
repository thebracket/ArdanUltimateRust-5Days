fn read_line() -> String {
    // <- Public function
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn parkable_thread(n: u32) {
    loop {
        std::thread::park();
        println!("Thread {n} is awake - briefly!");
    }
}

fn main() {
    let mut threads = Vec::new();
    for i in 0..10 {
        let thread = std::thread::spawn(move || parkable_thread(i));
        threads.push(thread);
    }

    loop {
        println!("Enter a thread number to awaken, or q to quit");
        let input = read_line();
        if input == "q" {
            break;
        }
        if let Ok(number) = input.parse::<u32>() {
            if number < threads.len() as u32 {
                threads[number as usize].thread().unpark();
            }
        }
    }
}
