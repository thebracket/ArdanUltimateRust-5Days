# Rust Fundamentals

Let's play around a bit in the hello world program, and learn some Rust fundamentals.

> You're encouraged to code along, but there will be some explanation time.

## Variables, Mutability and Shadowing

> These are included in the `code` directory, in the `vars_mut_shadow` project.

Let's add a variable to our program.

```rust
fn main() {
    let n = 5;
    println!("{n}");
}
```

You declare a variable with `let`. Variables are immutable by default. You can't do this:

```rust
fn main() {
    let n = 5;
    n = n + 1; // <-- Will not compile
    println!("{n}");
}
```

So how do I change `n`?

**Option 1: Make it mutable***

```rust
fn main() {
    let mut n = 5;
    n += 1;
    println!("{n}");
}
```

**Option 2: Replace it with shadowing**

```rust
fn main() {
    let n = 5;
    let n = n + 1;
    // Where did n go?
    println!("{n}");
}
```

`n` was *shadowed* by the new `n`. The first `n` is no longer accessible. This is a common pattern in Rust code, but you have to be careful. Don't accidentally create a new variable that replaces one you still need!

> Also: don't use single letter names in real code. Please! Future you will thank you.

## Scope Return, the Unit Type and Functions

Rust uses *scopes* everywhere. What is a scope? Anything between `{` and `}`. Variables declared inside a scope cease to exist when the scope finishes (and clean up after themselves!).

```rust
fn main() {
    let n = 5;
    {
        let n = 6;
        println!("{n}");
    }
    println!("{n}");
}
```

Shadowing obeys scopes! `n` in the outer scope is 5, and is 6 in the inner scope.

The last line of a function is the return value. You can assign a scope result to a variable:

```rust
fn main() {
    let n = {
        let mut accumulator = 0;
        for i in 1..10 {
            accumulator += i;
        }
        accumulator // No semicolon - this is the return value
    };
    println!("{n}");
}
```

Even scopes that don't return values, quietly *do* return something --- the Unit Type: `()`:

```rust
fn main() {
    let n = {
        println!("Hello World!");
    };
    println!("{n:?}"); // :? is a debug formatter
}
```

This returns `()` - a *unit type*. You usually won't need this, but it's important to understanding how Rust really works. It's mostly a functional language, under the hood. It also allows an error to be thrown when you try to use a value that doesn't exist.

So what about functions? Functions work just like scopes, but with a name. Functions can take parameters:

```rust
fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    let n = double(5);
    println!("{n}");
}
```

So `n` in the function is completely independent of `n` outside of the function, and ceases to exist (except as a return value) when the function ends.

You can use explicit return, also - at any point in the function:

```rust
fn double_or_nothing(n: i32) -> i32 {
    if n > 0 {
        return n * 2;
    }
    0
}

fn main() {
    println!("{}", double_or_nothing(5));
    println!("{}", double_or_nothing(-5));
    println!("{}", {
        let n = 12;
        return n;
    });
}
```

Notice that `return n` is the exception to the early return rule. Return *always* exits a *function* - not the scope block. Instead, use `n` (no semicolon) to return from a scope.

> Some people find explicit returns easier to read. Clippy - the linter - will complain. If you find it easier to read, use it. If you don't, don't.

## Move by Default - Except when you Copy

> The code for this is in the `function_returns` example.

This is confusing to a lot of people, so let's walk through the topic. Rust is a *move by default* language, and the borrow checker will complain if you try to use a variable after it's been moved. Let's see what that means:

```rust
fn greet(s: String) {
    println!("Hello, {s}");
}

fn main() {
    let mut name = "Hello".to_string();
    greet(name);
    name = name + " World"; // This won't compile!
}
```

The `n` we've been using was a *primitive type* - a built-in number. It's faster to copy these around, so they default to "copy" instead of "move". Just about every other type defaults to being a *move* type. When you pass it to a function, it's moved to the function, and you can't use it anymore - unless the function gives it back:

```rust
fn greet(s: String) -> String {
    println!("Hello, {s}");
    s
}

fn main() {
    let mut name = "Hello".to_string();
    name = greet(name);
    name = name + " World"; // This will compile
}
```

You can also *clone* types that support it:

```rust
fn greet(s: String) {
    println!("Hello, {s}");
}

fn main() {
    let mut name = "Hello".to_string();
    greet(name.clone());
    name = name + " World"; // This won't compile!
}
```

Cloning can be slow, so you don't want to do this everywhere.

## Borrowing & References

> The code for this is in the `borrow_reference` example.

So how do you pass a variable to a function, and still use it afterwards? You *borrow* it. Borrowing is a *reference* to the variable. You can borrow a variable with `&`:

```rust
fn greet(s: &String) {
    println!("Hello, {s}");
}

fn main() {
    let mut name = "Hello".to_string();
    greet(&name);
    name = name + " World";
}
```

Adding the ampersand (`&`) has *borrowed* the variable. The main function retains *ownership* - it's still main's variable, and it's main's responsibility to clean it up. But the function can use it, it isn't altered, and you've saved a clone---your just passing a pointer.

You can borrow mutably, too:

```rust
fn greet(s: &mut String) {
    *s = format!("Hello {s}");
    println!("{s}");
}

fn main() {
    let mut name = "Hello".to_string();
    greet(&mut name);
    name += " World";
    println!("{name}");
}
```

Notice that we are using `*` to *de-reference* the variable - point back at the original. You only need to do this if you aren't accessing a member of the variable.

You can only have one mutable borrow to a variable at a time. This becomes important for global variables and when you start using concurrency. The infamous "borrow checker" strictly enforces this rule to prevent data races.

