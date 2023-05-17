# Building a Login Manager App

We've already built a moderately useful login system: it can read users from a JSON file, creating a default if necessary. Logins are checked, passwords are hashed, and different login roles work. Let's spend the rest of our time together building a `login_manager` application that provides a command-line interface to our login system.

## Creating a New Project

Create a new `login_manager` project:

```bash
cargo new login_manager
```

Open the parent `Cargo.toml` and add `login_manager` to the workspace.

Now add the `auth` library to your `login_manager`'s `Cargo.toml` file:

```toml
[dependencies]
auth = { path = "../auth" }
```

## Creating a CLI

The de-facto standard approach to building CLI applications is provided by a crate named `clap`. Add it with:

```bash
cargo add clap -F derive
```

Clap does a *lot*, and the "derive" feature adds some useful macros to reduce the amount of typing we need to do.

Let's create a minimal example and have a look at what Clap is doing for us:

```rust
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
}


fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => {
            println!("All Users Goes Here\n");
        }
        None => {
            println!("Run with --help to see instructions");
            std::process::exit(0);
        }
    }    
}
```

This has added a surprising amount of functionality!

`cargo run` on its own emits `Run with --help to see instructions`. Clap has added `--help` for us.

Running cargo and then passing command-line arguments through uses some slightly strange syntax. Let's give `--help` a go:

```
cargo run -- --help

Usage: login_manager.exe [COMMAND]

Commands:
  list  List all users
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

You an even ask it for help about the `list` feature:

```
List all users

Usage: login_manager.exe list

Options:
  -h, --help  Print help
```

Now, let's implement the `list` command.

```rust
fn list_users() {
    println!("{:<20}{:<20}", "Username", "Login Action");
    println!("{:-<40}", "");

    let users = get_users();
    users
        .iter()
        .for_each(|(_, user)| {
            println!("{:<20}{:<20?}", user.username, user.role);
        });
}

fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => list_users(),
        None => {
            println!("Run with --help to see instructions");
            std::process::exit(0);
        }
    }    
}
```

Now running `cargo run -- list` gives us:

```
Username            Login Action        
----------------------------------------
admin               Admin
bob                 User
```

## Adding Users

We're going to need a way to save the users list, so in the auth library let's add a function:

```rust
pub fn save_users(users: &HashMap<String, User>) {
    let users_path = Path::new("users.json");
    let users_json = serde_json::to_string(&users).unwrap();
    std::fs::write(users_path, users_json).unwrap();
}
```

This is the same as what we did before---but exposed as a function.

Let's add an "add" option. It will have parameters, you need to provide a username, password and indicate if the user is an administrator:

```rust
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
  }
}
```

Add a dummy entry to the `match` statement:

```rust
Some(Commands::Add { username, password, admin }) => {},
```

And run `cargo run -- add --help` to see what Clap has done for us:

```
Add a user

Usage: login_manager.exe add [OPTIONS] <USERNAME> <PASSWORD>

Arguments:
  <USERNAME>  Username
  <PASSWORD>  Password

Options:
      --admin <ADMIN>  Optional - mark as an admin [possible values: true, false]
  -h, --help           Print help
```

Now we can implement the `add` command:

```rust
fn add_user(username: String, password: String, admin: bool) {
    let mut users = get_users();
    let role = if admin {
        LoginRole::Admin
    } else {
        LoginRole::User
    };
    let user = User::new(&username, &password, role);
    users.insert(username, user);
    save_users(&users);
}

fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => list_users(),
        Some(Commands::Add { username, password, admin }) => 
            add_user(username, password, admin.is_some()),
        None => {
            println!("Run with --help to see instructions");
            std::process::exit(0);
        }
    }    
}
```

And now you can run `cargo run -- add fred password` and see the new user in the list.

```json
{
    "fred": {
        "username": "fred",
        "password": "5E884898DA28047151D0E56F8DC6292773603D0D6AABBDD62A11EF721D1542D8",
        "role": "User"
    },
    "admin": {
        "username": "admin",
        "password": "5E884898DA28047151D0E56F8DC6292773603D0D6AABBDD62A11EF721D1542D8",
        "role": "Admin"
    },
    "bob": {
        "username": "bob",
        "password": "5E884898DA28047151D0E56F8DC6292773603D0D6AABBDD62A11EF721D1542D8",
        "role": "User"
    }
}
```

Let's add one more thing. Warn the user if a duplicate occurs:

```rust
fn add_user(username: String, password: String, admin: bool) {
    let mut users = get_users();
    if users.contains_key(&username) {
        println!("{username} already exists");
        return;
    }
```

## Deleting Users

Let's add a `delete` command. This will take a username and remove it from the list:

```rust
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
}
```

As expected, `--help and `cargo run -- delete --help` have been updated.

Now let's implement the deletion:

```rust
fn delete_user(username: &str) {
    let mut users = get_users();
    if users.contains_key(username) {
        users.remove(username);
        save_users(&users);
    } else {
        println!("{username} does not exist");
    }
}
```

And add it to the command matcher:

```rust
Some(Commands::Delete { username }) => delete_user(&username),
```

You can now remove fred from the list with `cargo run -- delete fred`. Check that he's gone with `cargo run -- list`:

```
Username            Login Action
----------------------------------------
bob                 User
admin               Admin
```

## Changing Passwords

You've got the Create, Read and Delete of "CRUD" - let's add some updating!

A command to change the user's password is in order. This will take a username and a new password:

```rust
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
```

And let's implement it:

```rust
fn change_password(username: &str, password: &str) {
    let mut users = get_users();
    if let Some(user) = users.get_mut(username) {
        user.password = auth_login_manager::hash_password(password);
        save_users(&users);
    } else {
        println!("{username} does not exist");
    }
}
```

And add it to the `match`:

```rust
Some(Commands::ChangePassword { username, new_password }) => {
    change_password(&username, &new_password)
}
```

Go ahead and test changing a password.