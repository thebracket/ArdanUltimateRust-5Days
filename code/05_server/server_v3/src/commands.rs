use once_cell::sync::Lazy;
use shared_v3::TaskType;
use std::sync::Mutex;
use std::collections::HashMap;

static COMMANDS: Lazy<Mutex<HashMap<u128, TaskType>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn add_command(collector_id: u128, command: TaskType) {
    let mut commands = COMMANDS.lock().unwrap();
    commands.insert(collector_id, command);
}

pub fn get_commands(collector_id: u128) -> Option<TaskType> {
    let mut commands = COMMANDS.lock().unwrap();
    commands.remove(&collector_id)
}