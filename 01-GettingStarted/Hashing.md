# Hashing Passwords

> The code for this section is in `auth_hash` and `login_hash`.

You probably don't want to be saving passwords in plain text. It's high on the list of "programming oopsies" that lead to security issues.

Instead, you should hash the password. Hashing is a one-way function that takes a string and returns a fixed-length string. It's not possible to reverse the process, so you can't get the original password back from the hash.

## Dependencies

We want to add another dependency, this time on the `sha2` crate. You can either run `cargo add sha2` or edit the `Cargo.toml` file yourself:

```toml
[package]
name = "auth_hash"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0.163", features = ["derive"] }
serde_json = "1.0.96"
sha2 = "0"
```

Now open `lib.rs` and add a new function to actually hash passwords:

```rust
pub fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}
```

## Hashing Passwords as Users are Added

We're creating the `users.json` file as needed, but we're not hashing the passwords. Let's fix that. If you have a `users.json` file, delete it - so we can start afresh.

Change the `User` constructor to automatically hash the password it is given:

```rust
impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        User {
            username: username.to_lowercase(),
            password: hash_password(password),
            role,
        }
    }
}
```

If you run the program, your login will fail - but you'll see that the `users.json` file now has a hashed password:

```json
{
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

That's definitely harder to guess! Now we can update the `login` function to hash the incoming password and compare it to the stored hash:

```rust
pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let users = get_users();
    let password = hash_password(password);

    if let Some(user) = users.get(username) {
        if user.password == password {
            Some(LoginAction::Granted(user.role.clone()))
        } else {
            Some(LoginAction::Denied)
        }
    } else {
        None
    }
}
```

We've added one line - replacing the `password` with a hashed version. Run the login program now, and it should work.