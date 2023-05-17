use auth_enum::{read_line, login, LoginAction};

fn main() {
    let mut tries = 0;
    loop {
        println!("Enter your username:");
        let username = read_line();
        println!("Enter your password:");
        let password = read_line();
        match login(&username, &password) {
            LoginAction::Admin => {
                println!("Welcome {username}, you are an admin.");
                break;
            }
            LoginAction::User => {
                println!("Welcome {username}, you are a regular user.");
                break;
            }
            LoginAction::Denied => {
                println!("Login failed.");
                tries += 1;
                if tries >= 3 {
                    println!("Too many failed attempts. Exiting.");
                    break;
                }
            }
        }
    }
}
