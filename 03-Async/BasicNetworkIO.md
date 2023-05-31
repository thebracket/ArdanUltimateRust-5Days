# Basic Network IO

There's synchronous versions of most network calls in the Rust Standard Library, but networking really lends itself to async: there's always going to be latency between calls (even if you have an enormous fiber feed!)

> The code for the first section is in `03_async/weather`.

## Making a REST call

Add three crates:

```bash
cargo add tokio -F full
cargo add reqwest -F json
cargo add anyhow
```

There are two popular crates to use for making HTTP calls: `reqwest` and `hyper`. `reqwest` is a higher-level crate that uses `hyper` under the hood. We'll use `reqwest` here.

Let's perform a very basic request to lookup the weather around my house (that's lat/lon for the city center):

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const URL: &str = "https://api.open-meteo.com/v1/forecast?latitude=38.9517&longitude=-92.3341&current_weather=true";
    let response = reqwest::get(URL).await?;
    println!("{}", response.text().await?);

    Ok(())
}
```

Notice that getting the response is an async call---we have to await it. Getting the body is *also* an async call. It may not be completely ready by the time we call `text()`, so we have to await that too.

The result is JSON:
```json
{"latitude":38.95111,"longitude":-92.335205,"generationtime_ms":0.16498565673828125,"utc_offset_seconds":0,"timezone":"GMT","timezone_abbreviation":"GMT","elevation":216.0,"current_weather":{"temperature":30.3,"windspeed":7.2,"winddirection":162.0,"weathercode":1,"is_day":1,"time":"2023-05-30T18:00"}}
```

You should remember how to parse JSON from the first class. Let's add `serde` support to our project:

```bash
cargo add serde -F derive
```

Let's build some strong types to represent the result:

```rust
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Weather {
    latitude: f64,
    longitude: f64,
    current_weather: CurrentWeather,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
}
```

Notice that we're just ignoring some fields altogether. That's ok. You can also make lines an `Option<String>` (or other type) if they may or may not be present.

Now we can use Reqwest's `json` feature to give us a strongly typed result:

```rust
const URL: &str = "https://api.open-meteo.com/v1/forecast?latitude=38.9517&longitude=-92.3341&current_weather=true";
let response = reqwest::get(URL).await?;
let weather: Weather = response.json().await?;
println!("{weather:#?}");
```

Right now, my weather looks like this:

```ron
Weather {
    latitude: 38.95111,
    longitude: -92.335205,
    current_weather: CurrentWeather {
        temperature: 30.3,
        windspeed: 7.2,
    },
}
```

That's all there is to making a basic HTTP(s) REST request! It's async, so it won't block your program.

If you find yourself dealing with less-structured JSON that doesn't readily lend itself to a strong type, Serde has your back. You can deserialize to a `serde_json::Value` type:

Run `cargo add serde_json` and then change the deserializer to:

```rust
let weather: serde_json::Value = response.json().await?;
```

This gives you a big collection of `Serde::Value` types that you can parse with iteration and matching:

```ron
Object {
    "current_weather": Object {
        "is_day": Number(1),
        "temperature": Number(30.3),
        "time": String("2023-05-30T18:00"),
        "weathercode": Number(1),
        "winddirection": Number(162.0),
        "windspeed": Number(7.2),
    },
    "elevation": Number(216.0),
    "generationtime_ms": Number(0.16701221466064453),
    "latitude": Number(38.95111),
    "longitude": Number(-92.335205),
    "timezone": String("GMT"),
    "timezone_abbreviation": String("GMT"),
    "utc_offset_seconds": Number(0),
}
```

## Making a Simple TCP Server

> The code for this is in `03_async/tcp_echo`.

(Once again, don't forget to add Tokio and Anyhow to your project!)

Let's create a very simple TCP server that simply echoes back anything you type. We'll use the `tokio::net::TcpListener` type to listen for connections, and `tokio::net::TcpStream` to handle the connection.

We'll start by using some of the types we need:
```rust
use tokio::{net::TcpListener, spawn, io::{AsyncReadExt, AsyncWriteExt}};
```

Then we'll build a main function that creates a "TCP Listener" to listen for new connections:

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8123").await?;
```

That's enough to listen for new connections on localhost, port 8123. You could use `::` for IPv6.

Now we're going to loop forever and accept new connections:

```rust
    loop {
        let (mut socket, address) = listener.accept().await?;
        spawn(async move {
            println!("Connection from {address:?}");
            // We're in a new task now. The task is connected
            // to the incoming connection. We moved socket (to talk to the other end)
            // and address (to print it's address) into this task.
            //
            // Because we used `spawn`, the task is added to the tasks pool and the
            // outer function continues to listen for new connections.
        });
    }
```

Now we'll fill in the blanks:

```rust
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    spawn,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8123").await?;

    loop {
        let (mut socket, address) = listener.accept().await?;
        spawn(async move {
            println!("Connection from {address:?}");
            let mut buf = vec![0; 1024];
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }

                socket
                    .write_all(&buf[0..n])
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
    //Ok(())
}
```

We then:

1. Initialize a buffer.
2. Loop forever.
3. Read from the socket into the buffer.
4. If we read 0 bytes, the connection is closed, so we return.
5. Otherwise, we write the buffer back to the socket.

If you `telnet` to `localhost:8123` and type some text, you'll see it echoed back to you.

That's not very useful, but it gives you one of the basic structures for accepting TCP connections yourself. We'll build a much better example later, and the final class of this course will build something useful!

## Making a Simple TCP Client

Let's build a client that connects to this server and verifies that it receives what it sent.

> The code for this is in `03_async/tcp_echo_client`.

Once again, add `tokio` and `anyhow` crates!

We'll start by creating a main function that connects to the server:

```rust
use tokio::{net::TcpStream, io::{AsyncWriteExt, AsyncReadExt}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8123").await?;
    println!("Connected to the server!");
```

The `TcpStream` provides an async low-level interface to a TCP stream. You can read/write bytes to it, managing buffers is up to you.

Now that we're connected, let's send "Hello World" to the server:

```rust
    // Send "Hello World"
    stream.write_all(b"Hello World!").await?;
```

Notice the `b"Hello World!"`. The `b` prefix means "this string is actually an array of bytes" - it's a handy bit of syntax sugar. it accepts a slice (reference to an array or vector) of bytes from anything.

Now let's read the response:

```rust
    // Read the response
    let mut buf = vec![0; 1024];
    let bytes_read = stream.read(&mut buf).await?;
    println!("Response: {}", String::from_utf8_lossy(&buf[..bytes_read]));

    Ok(())
}
```

Notice that we're using `from_utf8_lossy` to build the string. Rust strings aren't just byte streams like C---they are full UTF-8 unicode (a `char` can be more than one byte). If you try to build a string from a byte stream that isn't valid UTF-8, you'll get an error. `from_utf8_lossy` will replace invalid UTF-8 with a `?` character.

And when we run it (remember to start the server!):

```
Connected to the server!
Response: Hello World!
```

So that gives you the most basic scenarios to start working quickly: you can call REST services (`reqwest` also supports the other HTTP verbs), and you can build TCP servers and clients. We'll do more later!