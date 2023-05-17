use auth::{read_line, login_simple};

fn main() {
    let mut tries = 0;
    loop {
        println!("Enter your username:");
        let username = read_line();
        println!("Enter your password:");
        let password = read_line();
        if login_simple(&username, &password) {
            println!("Welcome, {username}!");
            break;
        } else {
            println!("Login failed.");
            tries += 1;
            if tries >= 3 {
                println!("Too many failed attempts. Exiting.");
                break;
            }
        }
    }
}