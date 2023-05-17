# First Program: Hello World

> This section is designed for you to code along with me, but will have some pauses for explanation.

## Create a new Rust Project

1. In a command prompt, navigate to the parent directory you wish to use. I'm going to be using `c:\users\herbe\Rust\LiveDay1` on my computer. You can use whatever you want, but I recommend you use a directory that is easy to find and remember.
2. Type `cargo new getting_started` and press enter. This will create a new directory called `getting_started` and initialize a new Rust project in it.

You should see `Created binary (application) getting_started package` appear.

Let's have a look at what's here:

* `.gitignore` - Cargo has made a Git repo for you. You'll only see this if you don't have a Git repo already created.
* `Cargo.toml` - the build system control
* `src` - The source code directory
  * `main.rs` - The main source file

Let's take a quick look at `main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

This is a very simple program. It has a `main` function---every executable needs one, it tells the OS where to start executing the program. It calls a *macro* --- the exclamation mark tells you that its a macro --- called `println`, which prints some text with a newline at the end.

Run the program with `cargo run`, and it predictably prints "Hello, world!". Congratulations! You've written your first Rust program.
