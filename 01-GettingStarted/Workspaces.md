# Cargo Workspaces

If you're working with more than one related project, `workspaces` can help you:

* Workspaces combine all of your builds into a single `target` folder. No need to find every single `target/` folder when cleaning up.
* Workspaces share downloads. If you have a bunch of projects with shared dependencies, workspaces build them once---and share the result. Faster compilation, less disk space usage.
* A lot of cargo commands can be run in the "workspace root" with `--all` as a command-line flag, and will operate on all of them. Run all of your tests with `cargo test --all`, or build everything with `cargo build --all`. Beware: `cargo run --all` really will try and run every program you've created in this workspace.

> While working on Hands-on Rust, I had so many examples outside of a workspace that I ran out of disk space and realized I was using hundreds of gigabytes with multiple copies of Legion and Bracket-Lib. Moving into a workspace meant I only had a single copy of each library, and was using a reasonable amount of disk space.

## Our Current Setup

Go back to your source folder for today. In my case `c:\users\herbert\rust\live`.

Open `Cargo.toml` and add a `workspace` section:

```toml
[workspace]
members = []
```

The workspace automatically includes the top-level `src` directory. This is the *parent*
workspace.

It's often confusing if you open a workspace with several projects in it and `cargo run` just
happens to run the first project you created!

Edit `src/main.rs` to change the message printed:

```rust
fn main() {
    println!("You probably wanted to run one of the nested workspaces.");
}
```

Now, if someone accidentally runs the parent project they are notified of their mistake.

Let's get back to where we were by creating a `login` project *inside* the workspace.

```
cd c:\users\herbert\rust\live
cargo new login
```

You will see a warning:

```
warning: compiling this new package may not work due to invalid workspace configuration

current package believes it's in a workspace when it's not:
current:   C:\Users\Herbert\Documents\Ardan\Rust Foundations 4 Day\src\hello_login_system\Cargo.toml
workspace: C:\Users\Herbert\Documents\Ardan\Rust Foundations 4 Day\Cargo.toml
error: failed to load manifest for workspace member `C:\Users\Herbert\Documents\Ardan\Rust Foundations 4 Day\hello_login_system`

Caused by:
  failed to read `C:\Users\Herbert\Documents\Ardan\Rust Foundations 4 Day\hello_login_system\Cargo.toml`

Caused by:
  The system cannot find the path specified. (os error 3)
```

> The paths will vary on your computer. In this case, it's referring to this Github repo.

This is a really long-winded way of saying: *Don't forget to add the new project to your workspaces' members*

Add the newly created project by re-opening the parent's `Cargo.toml` and adding it to your workspace:

```toml
members = [
    "login", # I find it helpful to include a note about what the project does, here.
]
```

## Try It Out

1. Change directory to your workspace root.
2. Type `cargo run`. You'll see the message that you probably didn't mean to run this one.
3. Change directory to your `login` program.
4. Type `cargo run` and see "Hello World".
5. Notice that there's only one `target` folder for the whole workspace. ALL compilation artifacts go here.

And that's it - you have a working workspace with all of its benefits.

> The GitHub version uses one master workspace for *all* the examples. You can open [/Cargo.toml](/Cargo.toml) to see a really large workspace in action!
