# Async File I/O

So you've come a long way: you can make a multi-threaded executor, spawn thousands of tasks and have them all perform asynchronously without context switches. You can even write unit tests for your async code and handle errors. That's great, but it hasn't gained you a lot: you can do all of that in synchronous code.

Where async really shines is handling IO. It's inevitable that there will be pauses while interacting with external devices. Async allows you to pause and allow other tasks to run. That's why Async shines for things like web servers.

For this example, grab the `code/03_async/buffered_reader/warandpeace.txt` file from the main GitHub repo. This is the entirety of *War and Peace* (Tolstoy), in a text file. It's a great example of a huge file!

## Buffered File I/O

Let's pretend that *War and Peace* is some corporate data that we actually want to work on. It's a big text file and we want to read it in and do some processing on it. We'll start with a synchronous version of the code:

```rust
// Taken from: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let now = std::time::Instant::now();
    let mut line_count = 0;
    if let Ok(lines) = read_lines("warandpeace.txt") {
        lines.for_each(|line| {
            if let Ok(line) = line {
                if !line.trim().is_empty() {
                    line_count += 1;
                }
            }
        });
    }
    println!("Read {} lines in {:.3} seconds", line_count, now.elapsed().as_secs_f32());
}
```

This is using some code from *Rust By Example* that I use a lot. The `read_lines` function is just too convenient to not copy! It uses a `BufReader` - so it's not loading the entire file into memory (which is a bad idea in many server contexts; `read_to_string` is actually substantially faster if you don't mind the memory overhead). It then uses the `lines` helper to iterate through each line.

On my computer (with a fast SSD), it completes in 0.031 seconds. That's fast, but it can be an eternity if you are blocking a thread---there's no yielding, and the the `BufReader` isn't asynchronous. Imagine your server is processing hundreds of these files at once!

Now let's try that in an async context. Don't forget to include Tokio and Anyhow:

```bash
cargo add tokio -F full
cargo add anyhow
```

Now let's change things up a little. We'll pass a filename into the function, and use Tokio:

```rust
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Taken from: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

async fn line_count(filename: String) -> anyhow::Result<usize> {
    let now = std::time::Instant::now();
    let mut line_count = 0;
    if let Ok(lines) = read_lines(filename) {
        lines.for_each(|line| {
            if let Ok(line) = line {
                if !line.trim().is_empty() {
                    line_count += 1;
                }
            }
        });
    }
    println!("Read {} lines in {:.3} seconds", line_count, now.elapsed().as_secs_f32());
    Ok(line_count)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Reading {filename}...");
    let now = std::time::Instant::now();
    let (c1, c2) = tokio::join!(
        line_count("warandpeace.txt".to_string()),
        line_count("warandpeace.txt".to_string())
    );
    println!("Total lines: {}", c1? + c2?);
    println!("In {:.3} seconds", now.elapsed().as_secs_f32());
    Ok(())
}
```

On my PC, I get the following output:

```
Reading warandpeace.txt...
Read 52162 lines in 0.032 seconds
Reading warandpeace.txt...
Read 52162 lines in 0.032 seconds
Total lines: 104324
In 0.066 seconds
```

Going `async` hasn't done us any good at all: the tasks block while the file i/o occurs, giving no performance improvement at all. The second task can't even start until the first one finishes. This is because the `BufReader` is not asynchronous.

## Async File I/O

Tokio provides a fair portion of the standard library in asynchronous form. Let's build an async version of the same function:

```rust
async fn async_line_count(filename: String) -> anyhow::Result<usize> {
    use tokio::io::AsyncBufReadExt;
    use tokio::io::BufReader;
    use tokio::fs::File;
```

So we've started by declaring an `async` function, and using the Tokio versions of the same types we were using before. Now let's open the file:

```rust
    println!("Reading {filename}...");
    let now = std::time::Instant::now();
    let mut line_count = 0;

    let file = File::open(filename).await?;
```

So now we do the same as before - indicate that we're doing something, initialize the timer and start counting lines. Then we open the file. The Tokio version is async - so you have to `await` the result (or spawn it, or join it, etc.). We're also using `?` to propagate errors, just like the synchronous version.

Now let's implement the buffered reader in async code. We'll keep it one function because it works a little differently:

```rust
    let reader = BufReader::new(file);
    let mut lines = reader.lines(); // Create a stream of lines
```

Instead of `lines` returning an *iterator* (like calling `.iter()` on vectors), it returns a *stream*. A stream is basically an async iterator---you `await` each iteration. Reading the next buffered section is implemented as an async operation, so the executor can yield if there's work to do while it's reading.

Let's read each entry and count it if the line isn't empty:

```rust
    while let Some(line) = lines.next_line().await? {
        if !line.trim().is_empty() {
            line_count += 1;
        }
    }

    println!("Read {} lines in {:.3} seconds", line_count, now.elapsed().as_secs_f32());
    Ok(line_count)
}
```

Now in `main` we can call the async version:

```rust
let now = std::time::Instant::now();
let (c1, c2) = tokio::join!(
    async_line_count("warandpeace.txt".to_string()),
    async_line_count("warandpeace.txt".to_string())
);
println!("Total lines: {}", c1? + c2?);
println!("In {:.3} seconds", now.elapsed().as_secs_f32());
```

We could keep adding readers, use different files, etc. and it would all work. The output is:

```
Reading warandpeace.txt...
Reading warandpeace.txt...
Read 52162 lines in 0.154 seconds
Read 52162 lines in 0.154 seconds
Total lines: 104324
In 0.155 seconds
```

The two readers have run concurrently, so you've read War And Peace twice without blocking!

> In all honesty, I haven't personally managed to read it once yet.

So when you are using `async` code, it's a good idea to use the `async` versions of the operations you are performing. You could also have used `spawn_blocking` to wrap the synchronous code in a thread.