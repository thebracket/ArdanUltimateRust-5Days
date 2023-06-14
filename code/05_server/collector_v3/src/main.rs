use std::collections::VecDeque;
use shared_v3::CollectorCommandV1;
mod data_collector;
mod sender;
mod errors;

fn get_uuid() -> u128 {
    let path = std::path::Path::new("uuid");
    if path.exists() {
        let contents = std::fs::read_to_string(path).unwrap();
        contents.parse::<u128>().unwrap()
    } else {
        let uuid = uuid::Uuid::new_v4().as_u128();
        std::fs::write(path, uuid.to_string()).unwrap();
        uuid
    }
}

fn main() {
    let uuid = get_uuid();
    let (tx, rx) = std::sync::mpsc::channel::<CollectorCommandV1>();

    // Start the collector thread
    let _collector_thread = std::thread::spawn(move || {
        data_collector::collect_data(tx, uuid);
    });

    // Listen for commands to send
    let mut send_queue = VecDeque::with_capacity(120);
    while let Ok(command) = rx.recv() {
        let encoded = shared_v3::encode_v1(&command);
        //println!("Encoded: {} bytes", encoded.len());
        send_queue.push_back(encoded);
        let result = sender::send_queue(&mut send_queue, uuid);
        if result.is_err() {
            println!("{result:?}");
        }
    }
}
