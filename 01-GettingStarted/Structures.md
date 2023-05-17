# Structures

You probably don't really want to have to write an `if` statement covering every user in your enterprise. Instead, you want to store usernames and passwords in a structure, along with the user's role.

## Basic Structures

> The code for this is found in `auth_struct` and `login_struct`.

You can define a simple structure like this:

```rust
pub struct User {
    pub username: String,
    pub password: String,
    pub role: Role,
}
```

Structs aren't object-oriented, but they share some commonality with objects from other languages. You can define methods on them, and you can define associated functions (functions that are called on the type, not on an instance of the type). Let's make a constructor:

```rust
impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        User {
            username: username.to_lowercase(),
            password: password.to_string(),
            role,
        }
    }
}
```

## Array of Structures

Let's create a function that creates an array of users:

```rust
pub fn get_users() -> [User; 2] {
    [
        User::new("admin", "password", LoginRole::Admin),
        User::new("bob", "password", LoginRole::User),
    ]
}
```

Arrays can never change in size, so with an array you are stuck with two users. Arrays do have the advantage of remaining on the stack, making them very fast to access.

Let's modify the `login` function to use this array:

```rust
pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let users = get_users();
    if let Some(user) = users.iter().find(|user| user.username == username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role));
        } else {
            return Some(LoginAction::Denied);
        }
    }
    None
}
```

`if let` works just like `match`, but for a single case. You can also write `match users.iter().find(|user| user.username == username && user.password == password) { Some(user) => LoginAction::from(user.role), None => LoginAction::Denied }` if you prefer.

This doesn't compile. Enumerations aren't copyable by default, because there's no guaranty that the contents are copyable. Add `#[derive(Clone)]` to the `Role` enumeration to make it clonable, and return a *clone* of the role:

```rust
pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let users = get_users();
    if let Some(user) = users.iter().find(|user| user.username == username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()));
        } else {
            return Some(LoginAction::Denied);
        }
    }
    None
}
```

We can test this with the login program, which hasn't changed.

