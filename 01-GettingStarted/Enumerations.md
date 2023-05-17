# Enumerations

You probably want more options than just "you are allowed in" and "you aren't permitted". You want to know *why* you aren't permitted. You want to know if the user is locked out, or if they have the wrong password, or if they are a new user and need to register. If the login succeeds, you want to know if they are an admin or a regular user.

Enumerations in Rust are *very* powerful---they are "algebraic data types" that can capture a lot of data. They are also "sum types"---meaning they only contain one of the types they are defined to contain.

## Basic Enumerations

> The code for this example is in `auth_enum` and `login_enum`.

Let's start with the most basic enumeration, which should be familiar from other languages:

```rust
pub enum LoginAction {
    Admin,
    User,
    Denied,
}
```

Now we can update the login function to return this enumeration:

```rust
pub fn login(username: &str, password: &str) -> LoginAction {
    let username = username.to_lowercase();
    if username == "admin" && password == "password" {
        LoginAction::Admin
    } else if username == "bob" && password == "password" {
        LoginAction::User
    } else {
        LoginAction::Denied
    }
}
```


And we can update the application to use it:

```rust
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
            break
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
```

> `match` is exhaustive: not matching a pattern will fail to compile. You can use `_` as a catch-all.

Let's add a unit test to the library:

```rust
#[test]
fn test_enums() {
    assert_eq!(login("admin", "password"), LoginAction::Admin);
    assert_eq!(login("bob", "password"), LoginAction::User);
    assert_eq!(login("admin", "wrong"), LoginAction::Denied);
    assert_eq!(login("wrong", "password"), LoginAction::Denied);
}
```

And everything goes red in the IDE! That's because enumerations don't support comparison by default. Fortunately, it's easy to fix. Let's support debug printing while we're at it:

```rust
#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Admin,
    User,
    Denied,
}
```

`#[derive]` is a procedural macro that writes code for you.

## Enumerations with Data

> The code for this section is in `login_enum_data` and `auth_enum_data`.

Let's clean up our enumerations a bit, and store some data in them:

```rust
#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug)]
pub enum LoginRole {
    Admin,
    User,
}

pub fn login(username: &str, password: &str) -> LoginAction {
    let username = username.to_lowercase();
    if username == "admin" && password == "password" {
        LoginAction::Granted(LoginRole::Admin)
    } else if username == "bob" && password == "password" {
        LoginAction::Granted(LoginRole::User)
    } else {
        LoginAction::Denied
    }
}
```

Now we can update the application to use the new data:

```rust
match login(&username, &password) {
    LoginAction::Granted(LoginRole::Admin) => {
        println!("Welcome {username}, you are an admin.");
        break;
    }
    LoginAction::Granted(LoginRole::User) => {
        println!("Welcome {username}, you are a regular user.");
        break
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
```

Notice how `match` lets you peer inside multiple levels of enumeration. This type of pattern matching is very useful.

# Optional Users

> The code for this is found in `auth_enum_option` and `login_enum_option`.

Maybe we want the login system to know that a user doesn't exist. You might want to offer suggestions, or an option to create a new user. You can do this with an `Option`. Options are an enumeration that either contain `Some(data)` or `None`. They use generics to store whatever type you want to put inside them - but they are a sum type, you are storing one or the other, never both.

```rust
pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();

    if username != "admin" && username != "bob" {
        return None;
    }

    if username == "admin" && password == "password" {
        Some(LoginAction::Granted(LoginRole::Admin))
    } else if username == "bob" && password == "password" {
        Some(LoginAction::Granted(LoginRole::User))
    } else {
        Some(LoginAction::Denied)
    }
}
```

Now we can update the login program to know if a user doesn't exist:

```rust
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
```

`match` allows for very deep pattern matching. You usually don't need to nest `match` statements.