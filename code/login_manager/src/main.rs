use auth_login_manager::{get_users, save_users, LoginRole, User};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all users.
    List,
    /// Add a user.
    Add {
        /// Username
        username: String,

        /// Password
        password: String,

        /// Optional - mark as an admin
        #[arg(long)]
        admin: Option<bool>,
    },
    /// Delete a user
    Delete {
        /// Username
        username: String,
    },
    /// Change a password
    ChangePassword {
        /// Username
        username: String,

        /// New Password
        new_password: String,
    },
}

fn delete_user(username: &str) {
    let mut users = get_users();
    if users.contains_key(username) {
        users.remove(username);
        save_users(&users);
    } else {
        println!("{username} does not exist");
    }
}

fn list_users() {
    println!("{:<20}{:<20}", "Username", "Login Action");
    println!("{:-<40}", "");

    let users = get_users();
    users.iter().for_each(|(_, user)| {
        println!("{:<20}{:<20?}", user.username, user.role);
    });
}

fn add_user(username: String, password: String, admin: bool) {
    let mut users = get_users();
    if users.contains_key(&username) {
        println!("{username} already exists");
        return;
    }
    let role = if admin {
        LoginRole::Admin
    } else {
        LoginRole::User
    };
    let user = User::new(&username, &password, role);
    users.insert(username, user);
    save_users(&users);
}

fn change_password(username: &str, password: &str) {
    let mut users = get_users();
    if let Some(user) = users.get_mut(username) {
        user.password = auth_login_manager::hash_password(password);
        save_users(&users);
    } else {
        println!("{username} does not exist");
    }
}

fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => list_users(),
        Some(Commands::Add {
            username,
            password,
            admin,
        }) => add_user(username, password, admin.is_some()),
        Some(Commands::Delete { username }) => delete_user(&username),
        Some(Commands::ChangePassword { username, new_password }) => {
            change_password(&username, &new_password)
        }
        None => {
            println!("Run with --help to see instructions");
            std::process::exit(0);
        }
    }
}
