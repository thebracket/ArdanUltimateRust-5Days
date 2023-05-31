# Axum - Tokio's Web Framework

Axum is a web framework built on top of Tokio. It is inspired by the likes of Rocket, Actix and Warp. It is lightweight and relatively easy to use. It also includes a number of features that make it a good choice for building enterprise web services.

## Hello Web

> This example is in `03_async/hello_web`

Let's make a really trivial webserver that just returns "Hello World" with no formatting, nor even HTML decoration.

To start, you need two services: `tokio` and `axum`. Add them to your `Cargo.toml`:

```bash
cargo add tokio -F full
cargo add axum
```

Now in our `main.rs` file, we can build a minimal webserver quite easily:

```rust
use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(say_hello_text));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn say_hello_text() -> &'static str {
    "Hello, world!"
}
```

Let's unpack this:

* `Router` is an Axum service that matches URLs to services. In this case, we're matching the root URL (`/`) to the `say_hello_text` service.
* `addr` is set with a `SocketAddr` to bind to localhost on port 3000.
* `axum::Server` uses the *builder pattern*
    * `bind` sets the address to bind to.
    * `serve` accepts the `Router` you created, and launches a webserver.
    * You have to `await` the server, because it's an asynchronous task.
    * `unwrap` is used to handle any errors that might occur.

The `say_hello_text` function is relatively straightforward. Don't worry about `'static` --- we'll talk about it in next week's talk about memory management, resource management and variables.

Run the program with `cargo run`. It won't output anything. Point a browser at `http://localhost:3000` and be amazed by the `Hello, world!` message.

You've made a basic webserver in 16 lines of code. It's also very fast---which is extra easy since it doesn't really do anything yet.

## Let's return "Hello World" as HTML

A webserver that doesn't return HTML is a bit odd, so let's turn "Hello World" into a proper HTML page.

```rust
use axum::response::Html;

async fn say_hello_html() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}
```

And change your route to call the new function. Run the program and go to `http://localhost:3000` again. You should see a big, bold "Hello, world!".

## HTML in Static Files

It's easier to write large amounts of HTML in a separate file. You can then import the file into your program. Let's do that.

First, in your `src` directory create a file named `hello.html`:

```html
<html>
<head>
    <title>Hello World</title>
</head>
<body>
    <p>Greetings, oh lovely world.</p>
</body>
</html>
```

> I'm not great at HTML!

Now, in your `main.rs` file, you can import the file and return it as HTML:

```rust
async fn say_hello_html_included() -> Html<&'static str> {
    const HTML: &str = include_str!("hello.html");
    Html(HTML)
}
```

Change the route again, and your file will be included when you run the webserver.

## HTML in Dynamic Files

There's a very real performance benefit to statically loading your pages, but it makes editing them a pain. Let's make a dynamic page that loads the HTML from a file.

This only requires a small change:

```rust
async fn say_hello_file() -> Html<String> {
    let path = Path::new("src/hello.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}
```

Now run your webserver. Change the HTML file and reload the page. You should see the changes.

> Note: You probably want some cache in the real world---but this is great for rapid development.

## Add a JSON Get Service

Let's add `serde` to our project with `cargo add serde -F derive`.

Now we'll add a structure and make it serializable:

```rust
#[derive(Serialize)]
struct HelloJson {
    message: String,
}
```

And we can add a handler function to return some JSON:

```rust
async fn say_hello_json() -> axum::Json<HelloJson> {
    axum::Json(HelloJson {
        message: "Hello, World!".to_string(),
    })
}
```

Lastly, we need to add a route to use it:

```rust
let app = Router::new()
    .route("/", get(say_hello_file))
    .route("/json", get(say_hello_json));
```

Now run the server, and connect to `http://localhost:3000/json`. You'll see a JSON response.

## Responding to Other Verbs

Axum supports `get`, `post`, `put`, `delete`, `head`, `options`, `trace`, `connect` and `patch`. Let's add a `post` route.

```rust
async fn say_hello_post() -> &'static str {
    "Hello, POST!"
}
```

Now add it to your routes:

```rust
let app = Router::new()
    .route("/", get(say_hello_file))
    .route("/json", get(say_hello_json))
    .route("/post", post(say_hello_post));
```

Let's update the HTML page to perform the POST for us:

```html
<html>

<head>
    <title>Hello World</title>
</head>

<body>
    <p>Greetings, oh lovely world.</p>
    <p id="result"></p>
</body>

<script>
    function doPost() {
        fetch('/post', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: ''
        })
            .then(response => response.text())
            .then(result => {
                document.getElementById('result').innerHTML = result;
            })
            .catch(error => {
                console.error('Error:', error);
            });

    }

    doPost();
</script>

</html>
```

> As you can see, I'm not a JavaScript programmer either!

The same techniques work for all of the HTTP verbs.