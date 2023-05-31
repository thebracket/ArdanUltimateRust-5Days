# Error Handling

Much of this section applies to both async and non-async code. Async code has a few extra considerations: you are probably managing large amounts of IO, and really don't want to stop the world when an error occurs!

## Rust Error Handling

In previous examples, we've used `unwrap()` or `expect("my message")` to get the value out of a `Result`. If an error occurred, your program (or thread) crashes. That's not great for production code!

> **Aside**: Sometimes, crashing *is* the right thing to do. If you can't recover from an error, crashing is preferable to trying to continue and potentially corrupting data.

### So what *is* a Result?

A `Result` is an `enum`, just like we covered in [week 1](../01-GettingStarted/Enumerations.md). It's a "sum type"---it can be one of two things---and *never* both. A `Result` is either `Ok(T)` or `Err(E)`. It's deliberately hard to ignore errors!

This differs from other languages:

| **Language** | **Description** | **Error Types** |
| --- | --- | --- |
| C | Errors are returned as a number, or even NULL. It's up to you to decipher what the library author meant. Convention indicates that returning `<0` is an error, and `>=0` is success. | `int` |
| C++ | Exceptions, which are thrown and "bubble up the stack" until they are caught in a `catch` block. If an exception is uncaught, the program crashes. Exceptions can have performance problems. Many older C++ programs use the C style of returning an error code. Some newer C++ programs use `std::expected` and `std::unexpected` to make it easier to handle errors without exceptions. | `std::exception`, `expected`, `int`, anything you like! |
| Java | Checked exceptions---which are like exceptions, but handling them is mandatory. Every function must declare what exceptions it can throw, and every caller must handle them. This is a great way to make sure you don't ignore errors, but it's also a great way to make sure you have a lot of boilerplate code. This can get a little silly, so you find yourself re-throwing exceptions to turn them into types you can handle. Java is also adding the `Optional` type to make it easier to handle errors without exceptions. | `Exception`, `Optional` |
| Go | Functions can return both an error type and a value. The compiler won't let you forget to check for errors, but it's up to you to handle them. In-memory, you are often returning both the value and an empty error structure. | `error` |
| Rust | Functions return an `enum` that is either `Ok(T)` or `Err(E)`. The compiler won't let you forget to check for errors, and it's up to you to handle them. `Result` is *not* an exception type, so it doesn't incur the overhead of throwing. You're always returning a value or an error, never both. | `Result<T, E>` |

So there's a wide range of ways to handle errors across the language spectrum. Rust's goal is to make it easy to work with errors, and hard to ignore them - without incurring the overhead of exceptions. **However** (there's always a however!), default standard-library Rust makes it harder than it should be.

## Strongly Typed Errors: A Blessing and a Curse!

> The code for this is in the `03_async/rust_errors1` directory.

Rust's errors are very specific, and can leave you with a *lot* of things to match. Let's look at a simple example:

```rust
use std::path::Path;

fn main() {
    let my_file = Path::new("mytile.txt");
    // This yields a Result type of String or an error
    let contents = std::fs::read_to_string(my_file);
    // Let's just handle the error by printing it out
    match contents {
        Ok(contents) => println!("File contents: {contents}"),        
        Err(e) => println!("ERROR: {e:#?}"),
    }
}
```

This prints out the details of the error:

```
ERROR: Os {
    code: 2,
    kind: NotFound,
    message: "The system cannot find the file specified.",
}
```

That's great, but what if we want to do something different for different errors? We can match on the error type:

```rust
match contents {
    Ok(contents) => println!("File contents: {contents}"),
    Err(e) => match e.kind() {
        std::io::ErrorKind::NotFound => println!("File not found"),
        std::io::ErrorKind::PermissionDenied => println!("Permission denied"),
        _ => println!("ERROR: {e:#?}"),
    },
}
```

The `_` is there because otherwise you end up with a remarkably exhaustive list:

```rust
match contents {
    Ok(contents) => println!("File contents: {contents}"),
    Err(e) => match e.kind() {
        std::io::ErrorKind::NotFound => println!("File not found"),
        std::io::ErrorKind::PermissionDenied => println!("Permission denied"),
        std::io::ErrorKind::ConnectionRefused => todo!(),
        std::io::ErrorKind::ConnectionReset => todo!(),
        std::io::ErrorKind::ConnectionAborted => todo!(),
        std::io::ErrorKind::NotConnected => todo!(),
        std::io::ErrorKind::AddrInUse => todo!(),
        std::io::ErrorKind::AddrNotAvailable => todo!(),
        std::io::ErrorKind::BrokenPipe => todo!(),
        std::io::ErrorKind::AlreadyExists => todo!(),
        std::io::ErrorKind::WouldBlock => todo!(),
        std::io::ErrorKind::InvalidInput => todo!(),
        std::io::ErrorKind::InvalidData => todo!(),
        std::io::ErrorKind::TimedOut => todo!(),
        std::io::ErrorKind::WriteZero => todo!(),
        std::io::ErrorKind::Interrupted => todo!(),
        std::io::ErrorKind::Unsupported => todo!(),
        std::io::ErrorKind::UnexpectedEof => todo!(),
        std::io::ErrorKind::OutOfMemory => todo!(),
        std::io::ErrorKind::Other => todo!(),
        _ => todo!(),            
    },
}
```

Many of those errors aren't even relevant to opening a file! Worse, as the Rust standard library grows, more errors can appear---meaning a `rustup update` run could break your program. That's not great! So when you are handling individual errors, you should always use the `_` to catch any new errors that might be added in the future.

## Pass-Through Errors

> The code for this is in the `03_async/rust_errors2` directory.

If you are just wrapping some very simple functionality, you can make your function signature match the function you are wrapping:

```rust
use std::path::Path;

fn maybe_read_a_file() -> Result<String, std::io::Error> {
    let my_file = Path::new("mytile.txt");
    std::fs::read_to_string(my_file)
}

fn main() {
    match maybe_read_a_file() {
        Ok(text) => println!("File contents: {text}"),
        Err(e) => println!("An error occurred: {e:?}"),
    }
}
```

No need to worry about re-throwing, you can just return the result of the function you are wrapping.

## The `?` Operator

We mentioned earlier that Rust doesn't have exceptions. It *does* have the ability to pass errors up the call stack---but because they are handled explicitly in `return` statements, they don't have the overhead of exceptions. This is done with the `?` operator.

Let's look at an example:

```rust
fn file_to_uppercase() -> Result<String, std::io::Error> {
    let contents = maybe_read_a_file()?;
    Ok(contents.to_uppercase())
}
```

This calls our `maybe_read_a_file` function and adds a `?` to the end. What does the `?` do?
* If the `Result` type is `Ok`, it extracts the wrapped value and returns it---in this case to `contents`.
* If an error occurred, it returns the error to the caller.

This is great for function readability---you don't lose the "flow" of the function amidst a mass of error handling. It's also good for performance, and if you prefer the "top down" error handling approach it's nice and clean---the error gets passed up to the caller, and they can handle it.

### What if I just want to ignore the error?

You *must* handle the error in some way. You *can* just call the function:

```rust
file_to_uppercase();
```

This will generate a compiler warning that there's a `Result` type that must be used. You can silence the warning with an underscore:

```rust
let _ = file_to_uppercase();
```

`_` is the placeholder symbol - you are telling Rust that you don't care. But you are *explicitly* not caring---you've told the compiler that ignoring the error is a conscious decision!

You can also use the `if let` pattern and simply not add an error handler:

```rust
if let Ok(contents) = file_to_uppercase() {
    println!("File contents: {contents}");
}
```

### What About Different Errors?

The `?` operator is great, but it requires that the function support *exactly* the type of error that you are passing upwards. Otherwise, in a strong-typed language you won't be able to ensure that errors are being handled.

Let's take an example that draws a bit from our code on day 1.

> The code for this is in the `03_async/rust_errors3` directory.

Let's add Serde and Serde_JSON to our project:

```bash
cargo add serde -F derive
cargo add serde_json
```

And we'll quickly define a deserializable struct:

```rust
use std::path::Path;
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    name: String,
    password: String,
}

fn load_users() {
    let my_file = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_file)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)?;
    Ok(users)
}
```

This isn't going to compile yet, because we aren't returning a type from the function. So we add a `Result`:

```rust
fn load_users() -> Result<Vec<User>, Error> {
```

Oh no! What do we put for `Error`? We have a problem! `read_to_string` returns an `std::io::Error` type, and `serde_json::from_str` returns a `serde_json::Error` type. We can't return both!

### Boxing Errors

You'll learn about the `Box` type and what `dyn` means next week. For now, `Box` is a *pointer*---and the `dyn` flag indicates that it its contents is *dynamic*---it can return any type that implements the `Error` trait. You'll learn about traits next week, too!

There's a lot of typing for a generic error type, but it works:

```rust
type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn load_users() -> GenericResult<Vec<User>> {
    let my_file = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_file)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)?;
    Ok(users)
}
```

This works with every possible type of error. Let's add a `main` function and see what happens:

```rust
fn main() {
    let users = load_users();
    match users {
        Ok(users) => {
            for user in users {
                println!("User: {}, {}", user.name, user.password);
            }
        },
        Err(err) => {
            println!("Error: {err}");
        }
    }
}
```

The result prints:
```
Error: The system cannot find the file specified. (os error 2)
```

You have the exact error message, but you really don't have any way to tell what went wrong programmatically. That may be ok for a simple program.

### Easy Boxing with Anyhow

There's a crate named `anyhow` that makes it easy to box errors. Let's add it to our project:

```bash
cargo add anyhow
```

Then you can replace the `Box` definition with `anyhow::Error`:

```rust
fn anyhow_load_users() -> anyhow::Result<Vec<User>> {
    let my_file = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_file)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)?;
    Ok(users)
}
```

It still functions the same way:

```
Error: The system cannot find the file specified. (os error 2)
```

In fact, `anyhow` is mostly just a convenience wrapper around `Box` and `dyn`. But it's a *very* convenient wrapper!

Anyhow does make it a little easier to return your own error:

```rust
#[allow(dead_code)]
fn anyhow_load_users2() -> anyhow::Result<Vec<User>> {
    let my_file = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_file)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)?;
    if users.is_empty() {
        anyhow::bail!("No users found");
    }
    if users.len() > 10 {
        return Err(anyhow::Error::msg("Too many users"));
    }
    Ok(users)
}
```

I've included the short-way and the long-way - they do the same thing. `bail!` is a handy macro for "error out with this message". If you miss Go-like "send any error you like", `anyhow` has your back!

> As a rule of thumb: `anyhow` is great in client code, or code where you don't really care *what* went wrong---you care that an error occurred and should be reported.

## Writing Your Own Error Types

Defining a full error type in Rust is a bit of a pain. You need to define a struct, implement the `Error` trait, and then implement the `Display` trait. You'll learn about traits next week, but for now you can think of them as "interfaces" that define what a type can do.

This is included in the `rust_errors3` project. We're just going to look at it, because the Rust community as a whole has decided that this is overly painful and does it an easier way!

```rust
#[derive(Debug, Clone)]
enum UsersError {
    NoUsers, TooManyUsers
}

use std::fmt;

impl fmt::Display for UsersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UsersError::NoUsers => write!(f, "no users found"),
            UsersError::TooManyUsers => write!(f, "too many users found"),
        }
    }
}
```

That's quite a lot of typing for an error! Pretty much nobody in the Rust world does this, unless you are in an environment in which you can't rely on external crates. Let's do the same thing with the `thiserror` crate:

```bash
cargo add thiserror
```

And then:
```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum UsersError {
    #[error("No users found")]
    NoUsers, 
    #[error("Too many users were found")]
    TooManyUsers
}
```

That's much easier!

---

## Mapping Errors

So let's use the new error type (`UsersError`):

```rust
fn work_with_my_error() -> Result<Vec<User>, UsersError> {
    let my_file = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_file)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)?;
    if users.is_empty() {
        Err(UsersError::NoUsers)
    } else if users.len() > 10 {
        Err(UsersError::TooManyUsers)
    } else {
        Ok(users)
    }
}
```

Oh dear - that doesn't compile! Why? Because `read_to_string` and `from_str` return errors that aren't your `UsersError`.

We're trying to make a production library here, and having well-defined errors makes for clearer control flow for our users. So we need to *map* the errors to a type we can handle. Let's add two more error types to our enumeration:

```rust
#[derive(Debug, Error)]
enum UsersError {
    #[error("No users found")]
    NoUsers, 
    #[error("Too many users were found")]
    TooManyUsers,
    #[error("Unable to open users file")]
    FileError,
    #[error("Unable to deserialize json")]
    JsonError(serde_json::Error),
}
```

Notice that we've added a tuple member for `JsonError` containing the actual error message. You might want to use it later, since it tells you why it couldn't deserialize the file.

Let's tackle our first `?`: reading the file:

```rust
let raw_text = std::fs::read_to_string(my_file).map_err(|_| UsersError::FileError)?;
```

We're using `map_err` on the function. It calls a function that receives the *actual* error as a parameter, and then returns a different type of error---the one we've created.

You can do the same for deserializing:

```rust
let users: Vec<User> = serde_json::from_str(&raw_text).map_err(UsersError::JsonError)?;
```

In this case, a Rust shorthand kicks in. This is the same as `map_err(|e| UsersError::JsonError(e))` - but because you're just passing the parameter in, a bit of syntax sugar lets you shorten it. Use the long-form if that's confusing (and Clippy - the linter - will suggest the short version).

So what have you gained here?

* You are now clearly defining the errors that come out of your library or program---so you can handle them explicitly.
* You've retained the inner error that might be useful, which might be handy for logging.
* You aren't messing with dynamic types and boxing, you are just *mapping* to an error type.
* You've regained control: YOU decide what's really an error, and how much depth you need to handle it.

## Back to Async!

The error handling so far has been generic and applies to everything you might write in Rust. Once you get into async land, handling errors becomes even more important. If you've written a network service, there might be hundreds or even thousands of transactions flying around---and you want to handle errors cleanly, without bringing down your enterprise service.

> The code for this is in `03_async/rust_errors_async`

We're going to make use of a few crates for this project:

```bash
cargo add tokio -F full
cargo add futures
cargo add anyhow
cargo add thiserror
```

### Top-Level Error Handling

You can make use of the `?` operator in `main` by returning a `Result` from `main`. You also need to return `Ok(())` at the end:

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Ok(())
}
```

This can let you write pretty clean-looking code and still cause the program to stop with an explicit error message:

```rust
async fn divide(number: u32, divisor: u32) -> anyhow::Result<u32> {
    if divisor == 0 {
        anyhow::bail!("Dividing by zero is a bad idea")
    } else {
        Ok(number / divisor)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    divide(5, 0).await?;
    Ok(())
}
```

> Note: It's much easier to use the `checked_div` function and return an error from that! This is for illustration.

Running this yields:

```
Error: Dividing by zero is a bad idea
error: process didn't exit successfully: `C:\Users\Herbert\Documents\Ardan\5x1 Day Ultimate Rust\code\target\debug\rust_errors_async.exe` (exit code: 1)
```

### Joining Fallible Futures

You can use the above pattern to simplify your error handling. What if you want to run *lots* of async operations, any of which may fail?

Let's try this:

```rust
let mut futures = Vec::new();
for n in 0..5 {
    futures.push(divide(20, n));
}
let results = futures::future::join_all(futures).await;
println!("{results:#?}");
```

The program doesn't crash, but the result from `join_all` is an array of `Result` types. You could iterate the array and keep the ones that worked, decide to fail because something failed, etc.

What if you'd like to transform `[Result, Result, Result]` - a list of results - into a single `Result[list]`? If anything failed, it all failed. There's an iterator for that!

```rust
// Condense the results! ANY error makes the whole thing an error.
let overall_result: anyhow::Result<Vec<u32>> = results.into_iter().collect();
println!("{overall_result:?}");
```

So now you can turn any failure into a program failure if you like:

```rust
let values = overall_result?; // Crashes
```

Or what if you'd like to just keep the good ones (and log the errors!):

```rust
// Separate the errors and the results
let mut errors = Vec::new();
let good: Vec<_> = results
    .into_iter()
    .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
    .collect();
println!("{good:?}");
println!("{errors:?}");
Ok(())
```

You can do whatever you like with the errors. Logging them is a good idea (you could even replace the `errors.push` with a log call), we'll handle that in `tracing`.

So that was a larger section, but you now have the basics you need to write fallible---it can fail---but reliable code. We'll use these techniques from now on.