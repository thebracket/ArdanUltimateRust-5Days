# World's Simplest Login System

> The code for this is in the `auth` and `login` projects.

Now that we have a library and an application that uses it, let's build the world's most primitive login system.

In the library:

```rust
pub fn login(username: &str, password: &str) -> bool {
    username == "admin" && password == "password"
}
```

And we'll test it:
```
#[test]
fn test_login() {
    assert!(login("admin", "password"));
    assert!(!login("admin", "wrong"));
    assert!(!login("wrong", "password"));
}
```

That looks good. But we haven't checked for case. The password should be case-sensitive, but do we really care if the username is Herbert or herbert?

```rust
pub fn login(username: &str, password: &str) -> bool {
    username.to_lowercase() == "admin" && password == "password"
}
```

Now, let's go to the application and use it:

```rust
fn main() {
    let mut tries = 0;
    loop {
        println!("Enter your username:");
        let username = read_line();
        println!("Enter your password:");
        let password = read_line();
        if login(&username, &password) {
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
```