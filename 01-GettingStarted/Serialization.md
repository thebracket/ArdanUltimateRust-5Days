# Serialization / Deserialization

You probably don't want to hand-type your list of users and recompile every time users change! You might use a local passwords file, or even a database. In this section, we'll look at how to serialize and deserialize data to and from a file.

> The code for this is in `auth_json` and `login_json`.

## Dependencies

Serde is the de-facto standard serialization/deserialization library. It's very flexible, and can be used to serialize to and from JSON, XML, YAML, and more. We'll use JSON here.

The first thing to do is to add some dependencies to your `auth` project.

You need the `serde` crate, with the *feature* `derive`. Run:

```
cargo add serde -F derive
```

You also need `serde_json`:

```
cargo add serde_json
```

These commands make your `Cargo.toml` file look like this:

```toml
[package]
name = "auth_json"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0.163", features = ["derive"] }
serde_json = "1.0.96"
```

## Making Data Serializable

Import the `Serialize` and `Deserialize` macros:

```rust
use serde::{Serialize, Deserialize};
```

Then decorate your types with `#[derive(Serialize, Deserialize)]`:

```rust
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}
```

The macros write all the hard code for you. The only requirement is that every type you are including must also support Serialize and Deserialize. You *can* implement traits and write the serialization by hand if you prefer - but it's very verbose.

Now let's change our "get_users" system to work with a JSON file.

## Serializing to JSON

First, we add a `get_default_users` function. If there isn't a users file, we'll use this to make one:

```rust
fn get_default_users() -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert("admin".to_string(), User::new("admin", "password", LoginRole::Admin));
    users.insert("bob".to_string(), User::new("bob", "password", LoginRole::User));
    users
}
```

Next, let's change the `get_users` function to look for a `users.json` file and see if it exists:

```rust
pub fn get_users() -> HashMap<String, User> {
    let users_path = Path::new("users.json");
    if users_path.exists() {
        // Load the file
        HashMap::new()
    } else {
        // Create a file and return it
        let users = get_default_users();
        let users_json = serde_json::to_string(&users).unwrap();
        std::fs::write(users_path, users_json).unwrap();
        users
    }
}
```

That's all there is to creating a JSON file! We use `serde_json::to_string` to convert our `users` HashMap into a JSON string, and then write it to the file. Run the program, and `users.json` will appear:

```json
{"bob":{"username":"bob","password":"password","role":"User"},"admin":{"username":"admin","password":"password","role":"Admin"}}
```

## Deserializing from JSON

Let's extend the `get_users` function to read from `users.json` if it exists:

```rust
pub fn get_users() -> HashMap<String, User> {
    let users_path = Path::new("users.json");
    if users_path.exists() {
        // Load the file
        let users_json = std::fs::read_to_string(users_path).unwrap();
        let users: HashMap<String, User> = serde_json::from_str(&users_json).unwrap();
        users
    } else {
        // Create a file and return it
        let users = get_default_users();
        let users_json = serde_json::to_string(&users).unwrap();
        std::fs::write(users_path, users_json).unwrap();
        users
    }
}
```

Equally simple - you load the file, deserialize it with `serde_json::from_str`, and you're done! You can now edit the JSON file, and your changes will be loaded when a user tries to login.

Let's change admin's password to `password2` and test it.