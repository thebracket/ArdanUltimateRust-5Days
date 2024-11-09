# Sharing Data with Read/Write Locks

It's a really common pattern to have some data that changes infrequently, mostly accessed by worker threads---but occasionally, you need to change it.

> The code for this is in `rwlock` in the `code/02_threads` directory.

We're going to use `once_cell` in this example, so add it with `cargo add once_cell`. `once_cell` is on its way into the standard library.

Let's build a simple example of this in action:

```rust
use std::sync::RwLock;
use once_cell::sync::Lazy;

static USERS: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(build_users()));

fn build_users() -> Vec<String> {
    vec!["Alice".to_string(), "Bob".to_string()]
}

// Borrowed from last week!
pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    std::thread::spawn(|| {
        loop {
            println!("Current users (in a thread)");
            let users = USERS.read().unwrap();
            println!("{users:?}");
            std::mem::drop(users); // Release the lock before sleeping
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    });

    loop {
        println!("Enter a name to add to the list (or 'q' to quit):");
        let input = read_line();
        if input == "q" {
            break;
        }
        let mut users = USERS.write().unwrap();
        users.push(input);
    }
}
```

Notice that we've used the `Lazy` pattern we talked about last week: the static variable is only initialized when someone looks at it.

We've wrapped the list of names in an `RwLock`. This is like a `Mutex`, but you can either *read* or *write* to it. You can have multiple readers, but only one writer.

Uncontested read is very fast. Pausing for a write is very slightly slower than a `Mutex`, but not by much.