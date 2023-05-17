# Create a Library

**Terminology Time**

* A `library` is a generic term for shared code.
* A `crate` is a Cargo-managed bundle of code, often a library.
* A `module` is a section inside a crate.

These are often thrown around somewhat interchangeably - be careful!

## Create a new library

Go to your workspace root (`c:\users\herbert\rust\live\login_system` in my case):

```
cargo new --lib authentication
```
(`authentication` is the name of the library to create)

Once again, you'll get a big warning that really means *don't forget to update your workspace members*. Let's do that:

```toml
members = [
    "login", # Test login program
    "authentication", # Authenticator library
]
```

> You can find the code referred to in this section [here](/src/hello_auth/).

Cargo has once again created a `Cargo.toml` file, which is pretty much unchanged:

```toml
[package]
name = "hello_auth"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

The only change is the package name.

Cargo has also created a `lib.rs` file instead of a `main.rs` file. As we said earlier, this makes the package a *library* --- you can't run it, it's designed to be used from other applications. The content of `main.rs` is *very* different:

```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

Cargo has created:
* A single function named `add` that simply adds two parameters and returns the result.
* An entire unit test system, that asserts that `2+2` still equals `4`.

## Understanding Rust Function Syntax

Here's an annotated version of the `add` function:

```rust
//     ⬇️ The function name
//          ⬇️ Function parameters
//                                    ⬇️ The function return type
//⬇️ `pub` means "public" - the function is available from outside the library/module.
pub fn add(left: usize, right: usize) -> usize {
    left + right // No "return" statement is necessary. Rust functions return the last
                 // expression result from a function.
}
```

If you prefer `return`, this is also valid:

```rust
pub fn add(left: usize, right: usize) -> usize {
    return left + right;
}
```

Notice that Clippy complains when I do this. Canonical Rust uses the short form. It's important to know that you *can* use the `return` command---and you can do it anywhere in a function. Early return is often preferred.

> Enabling Clippy as the default in [the IDE setup](./setup_ide.md) makes Clippy bug you as you type. I prefer it this way. Clippy helps you write "Rustacean" code, and knows about a LOT of common things that could be done better. If Clippy is annoying you, you can switch back to `cargo check` mode.

## Understanding the Tests

The mysterious line:

```rust
#[cfg(test)]
```

Is a *compiler directive*. It tells Cargo (and `rustc` underneath) to *only* compile the next section if you are compiling in the "test" configuration. Your unit tests won't take up *any* extra space in a deployed version of your library.

```rust
mod tests {
    use super::*;
    ..
}
```

`mod tests` is declaring a *module*. Modules can also be declared as files and directories (you'll see that later). They serve as a namespace (so everything inside is `tests::name`), a scope (variables inside a module aren't visible from outside without `pub`, and won't pollute the namespaces of other modules), and often a compilation unit - modules can often be compiled in parallel.

You draw elements from other modules into the current one with `use`. `use super::*` means "use everything from the parent module."

Finally, the test itself:

```rust
#[test]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}
```

You can annotate functions, structures, enums and lots of other things in Rust with `#[..]` annotations. These apply to whatever is defined next. In this case, marking a function as a `test` adds it to Cargo's unit-test runner.

`add` only works because we imported everything from `super`. You could also write `super::add(2, 2);`.

`assert_eq!` is an assertion macro that panics your program if the two arguments are not equal.

Once again, you can use the `Command Palette` to run `Expand Macros` and see what it actually does:

```rust
match (&result, &4) {
    (left_val, right_val) => {
        if !(*left_val == *right_val) {
            let kind = $crate::panicking::AssertKind::Eq;
            $crate::panicking::assert_failed(
                kind,
                &*left_val,
                &*right_val,
                $crate::option::Option::None,
            );
        }
    }
}
```

## Run the Tests

So that's a very long-winded check that 2+2 equals 4. Let's run it. You can run unit tests for the current workspace entry at any time by typing:

```
cargo test
```

You'll see something like this:

```
   Compiling hello_auth v0.1.0 (C:\Users\Herbert\Documents\Ardan\Rust Foundations 4 Day\src\hello_auth)
    Finished test [unoptimized + debuginfo] target(s) in 0.31s
     Running unittests src\lib.rs (C:\Users\Herbert\Documents\Ardan\Rust Foundations 4 Day\target\debug\deps\hello_auth-9e01bca15e1e38fb.exe)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests hello_auth

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

**Good news: 2+2 does equal 4.**

You can also test your whole workspace at once by typing:

```
cargo test --all
```

## The Hello World Service

Let's create a "hello world" service.

> This is in the Github repo [here](/src/hello_as_a_service/)

### Step 1: Cleanup the Default Code

Delete the `add` function, and the unit test (keeping the framework).

Your code now looks like this:

```rust

#[cfg(test)]
mod tests {
    use super::*;
}
```

> Clippy complains that we aren't using any functions from `super`. That's ok for now.

### Step 2: Add a function

Now we'll add a function:
```rust
pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
}
```

There's a few things to notice here:

* We made the function *public* with `pub`. That means we can call it from programs that use the library.
* Rust has TWO types of string!
    * `&str` is an immutable buffer of characters in memory.
        * You usually use this for literals, such as `"Herbert"`.
        * You can refer to any `String` as an `&str` by borrowing it - with `&my_string`.
    * `String` is an all-singing, all dancing buffered string designed for modification.
        * Internally, `String` is a buffer of characters with the length stored.
        * Changing a `String` updates or replaces the buffer.
    * `format!` is another macro, used under the hood by `printn!` and other Rust formatters. [It's very powerful](https://doc.rust-lang.org/std/macro.format.html) - it can do formatting and placeholders.

### Step 3: Test the New Function

Before we integrate this with our main program, let's add a unit test to ensure that it does what we think:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!("Hello Herbert", greet_user("Herbert"));
    }
}
```

And run it with `cargo test`:

```
running 1 test
test tests::test_greet_user ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### Step 3: Integration

> This is in the Github repo [here](/src/hello_service_exe/)

Now we return to the `login` project (`c:\users\herbert\rust\live\login` in my case).

Open `Cargo.toml`, and let's add our project as a dependency:

```toml
[dependencies]
authentication = { path = "../authentication" }
```

> You can specify *version numbers* (e.g. `library = "1"` using semantic versioning. Provide as many significant numbers as you need; "1" will load the latest in the "1.x.y" tree. You can also use `{ git = "git path" }` and load directly from a shared Git repository. Useful if you are working with a team, or want to grab some code from Github.)

We'll update `main.rs` to use our new function:

```rust
use authentication::greet_user;

fn main() {
    println!("{}", greet_user("Herbert"));
}
```

Run it with `cargo run`:

```
   Compiling hello_service_exe v0.1.0 (C:\Users\Herbert\Documents\Ardan\Rust Foundations 4 Day\src\hello_service_exe)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `C:\Users\Herbert\Documents\Ardan\Rust Foundations 4 Day\target\debug\hello_service_exe.exe`
Hello Herbert
```

*Congratulations, you've made and used your first library.*