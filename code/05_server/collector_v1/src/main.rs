use shared_v1::CollectorCommandV1;
mod data_collector;
mod sender;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<CollectorCommandV1>();

    // Start the collector thread
    let _collector_thread = std::thread::spawn(move || {
        data_collector::collect_data(tx);
    });

    // Listen for commands to send
    while let Ok(command) = rx.recv() {
        sender::send_command(command);
    }
}
