# Tokio Tracing

Tokio includes a handy suite of tracing systems, suitable for enterprise application development and maintenance.

## Logging

> The code for this is in `03_async/tokio_tracing`

Let's add *four* dependencies:

```bash
cargo add tokio -F full
cargo add tracing
cargo add tracing-subscriber
cargo add anyhow
```

Here's a quick example of using Tokio Tracing for some logging:

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Applications that receive events need to subscribe
    let subscriber = tracing_subscriber::FmtSubscriber::new();

    // Set the subscriber as the default
    tracing::subscriber::set_global_default(subscriber)?;

    // Log some events
    tracing::info!("Starting up");
    tracing::warn!("Are you sure this is a good idea?");
    tracing::error!("This is an error!");

    Ok(())
}
```

Walking through this:
1. You have to setup a *subscriber* to receive events. Libraries won't do this, but applications will.
2. You can use the `FmtSubscriber` to log to the console.
3. You can set the subscriber as the default.
4. You use the `tracing` macros to log events.

The output is quite colorful, but GitHub doesn't do color:

```
2023-05-31T02:32:05.988738Z  INFO tokio_tracing: Starting up
2023-05-31T02:32:05.988981Z  WARN tokio_tracing: Are you sure this is a good idea?
2023-05-31T02:32:05.989203Z ERROR tokio_tracing: This is an error!
```

There are definite performance advantages to using tracing rather than `println!`. `println!` globally locks `stdout`, while the tracing macros aim for high speed.

## Configuring Output

There's lots of tracing options (from the Tokio Tracing documentation):

```rust
// Start configuring a `fmt` subscriber
let subscriber = tracing_subscriber::fmt()
    // Use a more compact, abbreviated log format
    .compact()
    // Display source code file paths
    .with_file(true)
    // Display source code line numbers
    .with_line_number(true)
    // Display the thread ID an event was recorded on
    .with_thread_ids(true)
    // Don't display the event's target (module path)
    .with_target(false)
    // Build the subscriber
    .finish();
```

Let's try this with everything displayed:

```
2023-05-31T02:35:26.815762Z  INFO ThreadId(01) 03_async\tokio_tracing\src\main.rs:25: Starting up
2023-05-31T02:35:26.816023Z  WARN ThreadId(01) 03_async\tokio_tracing\src\main.rs:26: Are you sure this is a good idea?
2023-05-31T02:35:26.816182Z ERROR ThreadId(01) 03_async\tokio_tracing\src\main.rs:27: This is an error!
```

If you think back to when we started, you could even name your threads!

## Performance Spans

You can also use tracing to monitor performance as the application runs. No more `Instant::now` and `elapsed()`!

You can decorate a function to be timed as follows:

```rust
#[tracing::instrument]
async fn hello_world() {
    println!("hello, world!");
}
```

When you build your subscriber, you can enable performance spans:

```rust
.with_span_events(FmtSpan::ENTER | FmtSpan::EXIT)
```

And you'll get output like this:

```
2023-05-31T02:40:31.232000Z  INFO ThreadId(01) hello_world: 03_async\tokio_tracing\src\main.rs:3: enter
hello, world!
2023-05-31T02:40:31.232195Z  INFO ThreadId(01) hello_world: 03_async\tokio_tracing\src\main.rs:3: close time.busy=191µs time.idle=13.3µs
```

That's great for quick benchmarking. You can trace spans within spans.

You can also configure spans to display extra information in tracing:

```rust
#[tracing::instrument(name = "doubler", fields(n))]
async fn double(n: u32) -> u32 {
    n * 2
}
```

Will show something like:

```
2023-05-31T02:43:42.602176Z  INFO ThreadId(01) doubler: 03_async\tokio_tracing\src\main.rs:15: enter n=4 number=4
2023-05-31T02:43:42.602732Z  INFO ThreadId(01) doubler: 03_async\tokio_tracing\src\main.rs:15: close time.busy=546µs time.idle=13.3µs n=4 number=4
```

See [this page](https://tokio.rs/tokio/topics/tracing) for even more information on what you can trace!

## Tokio-Console

Tokio also includes a console for real-time monitoring of Tokio tasks. It's not a full-blown profiler, but it's a handy tool for debugging.

> The code for this is in `03_async/tokio_console_demo`

## Setup

Start by adding 4 dependencies:

```bash
cargo add tokio -F full
cargo add console-subscriber
cargo add tracing
cargo add anyhow
```

### If You Aren't Using Workspaces

Then edit `Cargo.toml` to include the following:

```toml
[build]
rustflags = ["--cfg", "tokio_unstable"]
```

(Or edit the workspace's `Cargo.toml` - build flags are shared by the whole workspace)

### If you Just Want to Instrument This Project

Alternatively, you can run the program with:
* `RUSTFLAGS="--cfg tokio_unstable" cargo build` (on *NIX likes)
* `$env:RUSTFLAGS = '--cfg tokio_unstable'` and then `cargo build` (on Windows)

Next, you need to install the program itself. `Cargo` can also act as an installer for software on your computer:

```bash
cargo install --locked tokio-console
```

## Using Tokio Console

The `tokio_console_demo` in the source code is the same as our `tcp_echo` program from earlier, but with added instrumentation and the `tcp_echo_client` integrated to run every few milliseconds.

Run the console with:

```bash
tokio-console
```

Or for pretty colors:

```bash
tokio-console --colorterm truecolor
```

And run your program. You can now see what's happening!

## OpenTelemetry and Other Tracing Systems

There's a lot of Tokio tracing subscribers available. One that may be of interest is [OpenTelemetry](https://github.com/tokio-rs/tracing-opentelemetry). If you'd like to include your application in OpenTelemetry, everything you need is there.