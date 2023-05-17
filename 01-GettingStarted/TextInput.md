# Text Input

> The code for this is in the `text_input` project.

Let's create a function that will read a line of text from the console.

```rust
fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}
```

What's the `expect`? Accessing standard input might fail - so Rust is returning a `Result` type. We're going to look at those later. For now, we're just going to crash if it fails. You can also use `unwrap`, but `expect` lets you specify an error message.

Now let's test it:

```rust
fn main() {
    let input = read_line();
    println!("You typed: [{input}]");
}
```

Notice how there's an extra carriage return. Reading input keeps the control characters. This is rarely what you want, so let's trim it:

```rust
fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
```

Now let's test it:

```rust
fn main() {
    let input = read_line();
    println!("You typed: [{input}]");
}
```

Bingo - trimmed text input.