use auth_json::{login, LoginAction, LoginRole, read_line};

fn main() {
    let mut tries = 0;
    loop {
        println!("Enter your username:");
        let username = read_line();
        println!("Enter your password:");
        let password = read_line();
        match login(&username, &password) {
            Some(LoginAction::Granted(LoginRole::Admin)) => {
                println!("Welcome {username}, you are an admin.");
                break;
            }
            Some(LoginAction::Granted(LoginRole::User)) => {
                println!("Welcome {username}, you are a regular user.");
                break
            }
            Some(LoginAction::Denied) => {
                println!("Login failed.");
                tries += 1;
                if tries >= 3 {
                    println!("Too many failed attempts. Exiting.");
                    break;
                }
            }
            None => {
                println!("User does not exist.");
                break;
            }
        }
    }
}
