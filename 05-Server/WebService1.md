# Building a Web Service

We've got data flowing from collector(s) to the server. We aren't storing any data, and we aren't doing anything with it---but we're getting it there.

Let's start by addressing data storage.

> The code for this is in `code/05_server/server_v2`. I'm breaking the development into chunks in the curriculum so you can see the progression.

## Adding SQLX

I didn't want to ask everyone to install Influx (or another time-series database), which is the optimal way to handle this type of data. So instead, we're going to use SQLite.

If you don't already have it installed, add the SQLx command-line client:

```bash
cargo install sqlx-cli
```

Now we need to tell sqlx where our database is. Create a `.env` file in the `server` directory:

```bash
DATABASE_URL="sqlite:collection.db"
```

Create the database:

```bash
sqlx database create
```

And create an initial migration:

```bash
sqlx migrate add initial
```

## Build the Database

Open the migration file in `server/migrations/` and add the following SQL:

```sql
CREATE TABLE IF NOT EXISTS timeseries
(
    id SERIAL PRIMARY KEY,
    collector_id VARCHAR(255),
    received TIMESTAMP,
    total_memory UNSIGNED BIG INT,
    used_memory UNSIGNED BIG INT,
    average_cpu FLOAT
)

```

You can apply this (to verify your SQL) by running:

```bash
sqlx migrate run
```

## Add SQLX to the Server

The server will need SQLX with SQLite support added to its dependencies:

```bash
cargo add sqlx -F runtime-tokio-native-tls -F sqlite
```

We're going to want to validate our UUIDs and turn them into strings. Let's use the UUID crate for that:

```bash
cargo add uuid
```

We're also going to need `dotenv` to read the `.env` file:

```bash
cargo add dotenv
```

Now your dependency list looks like this:

```toml
tokio = { version = "1.28.2", features = ["full"] }
shared_v2 = { path = "../shared_v2" }
anyhow = "1.0.71"
sqlx = { version = "0.6.3", features = ["runtime-tokio-native-tls", "sqlite"] }
uuid = "1.3.3"
dotenv = "0.15.0"
```

### Add the Database Connection

In `main.rs`, add the following:

```rust
// Read the .env file and obtain the database URL
dotenv::dotenv()?;
let db_url = std::env::var("DATABASE_URL")?;

// Get a database connection pool
let pool = sqlx::SqlitePool::connect(&db_url).await?;
```

### Store Data

Change your `data_collector` function to accept a connection pool as a parameter. It then clones
it for each connection, and sends that to the connection handler.

```rust
pub async fn data_collector(cnn: Pool<Sqlite>) -> anyhow::Result<()> {
    // Listen for TCP connections on the data collector address
    let listener = TcpListener::bind(DATA_COLLECTOR_ADDRESS).await?;

    // Loop forever, accepting connections
    loop {
        // Wait for a new connection
        let cnn = cnn.clone();
        let (socket, address) = listener.accept().await?;
        tokio::spawn(new_connection(socket, address, cnn));
    }
}
```

The connection handler gains some additional code to store your data as it arrives:

```rust
async fn new_connection(mut socket: TcpStream, address: SocketAddr, cnn: Pool<Sqlite>) {
    let mut buf = vec![0u8; 1024];
    loop {
        let n = socket
            .read(&mut buf)
            .await
            .expect("failed to read data from socket");

        if n == 0 {
            println!("No data received - connection closed");
            return;
        }

        let received_data = decode_v1(&buf[0..n]);

        match received_data {
            (timestamp, CollectorCommandV1::SubmitData { collector_id, total_memory, used_memory, average_cpu_usage }) => {
                let collector_id = uuid::Uuid::from_u128(collector_id);
                let collector_id = collector_id.to_string();

                let result = sqlx::query("INSERT INTO timeseries (collector_id, received, total_memory, used_memory, average_cpu) VALUES ($1, $2, $3, $4, $5)")
                    .bind(collector_id)
                    .bind(timestamp)
                    .bind(total_memory as i64)
                    .bind(used_memory as i64)
                    .bind(average_cpu_usage)
                    .execute(&cnn)
                    .await;

                if result.is_err() {
                    println!("Error inserting data into the database: {result:?}");
                }
            }
        }        
    }
}
```

We're still running one task per connection, and that's good. Tokio will balance load between tasks for us, and we won't accept another item of data until we're ready for it. We're still accepting new connections.

If you run the program now (and the collector!), the database will grow---data is arriving.

## Axum REST Service

We'll use the Tokio project's `Axum` web server once again.

### Setup Axum

Add `Axum` to your server:

```bash
cargo add axum
```

Let's also add `futures` for its stream helpers:

```bash
cargo add futures
```

And in `main.rs`, we'll build a skeleton to start the web service.

```rust
use std::net::SocketAddr;
use axum::{Router, routing::get};
```

And to make a basic test work:

```rust
    // Start the web server
    let app = Router::new()
        .route("/", get(test))
        .layer(Extension(pool));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    // Wait for the data collector to finish
    handle.await??; // Two question marks - we're unwrapping the task result, and the result from running the collector.
    Ok(())
}

async fn test() -> &'static str {
    "Hello, world!"
}
```

Run the server now, and you can go to [http://localhost:3000](http://localhost:3000) and see the message.

### Build a REST API

Create a new file, `src/api.rs` and add `mod api;` to `main.rs`.

Let's build a basic "show everything" function. Initially, we'll just print to the console:

```rust
use axum::Extension;
use sqlx::FromRow;
use futures::TryStreamExt;

#[derive(FromRow, Debug)]
pub struct DataPoint {
    id: i32,
    collector_id: String,
    received: i64,
    total_memory: i64,
    used_memory: i64,
    average_cpu: f32,
}

pub async fn show_all(Extension(pool): Extension<sqlx::SqlitePool>) {
    let mut rows = sqlx::query_as::<_, DataPoint>("SELECT * FROM timeseries")
        .fetch(&pool);

    while let Some(row) = rows.try_next().await.unwrap() {
        println!("{:?}", row);
    }
}
```

In `main.rs`, add a route to it:

```rust
let app = Router::new()
    .route("/", get(test))
    .route("/api/all", get(api::show_all))
    .layer(Extension(pool));
```

Now run the server, and connect to [http://localhost:3000/api/all](http://localhost:3000/api/all). You should see a list of data points.

### Add a JSON Response

You'll need to add Serde with the `derive` feature:

```bash
cargo add serde -F derive
```

Using `serde` actually *shortens* our API call:

```rust
pub async fn show_all(Extension(pool): Extension<sqlx::SqlitePool>) -> Json<Vec<DataPoint>> {
    let rows = sqlx::query_as::<_, DataPoint>("SELECT * FROM timeseries")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(rows)
}
```

And if you run the server now and connect to [http://localhost:3000/api/all](http://localhost:3000/api/all), you'll see a JSON response.

## Adding Some More Commands

Let's add a few more commands to our API.

### Listing Collectors

Let's list just the collectors we know about

```rust
#[derive(FromRow, Debug, Serialize)]
pub struct Collector {
    id: i32,
    collector_id: String,
    last_seen: i64,
}

pub async fn show_collectors(Extension(pool): Extension<sqlx::SqlitePool>) -> Json<Vec<Collector>> {
    const SQL: &str = "SELECT 
    DISTINCT(id) AS id, 
    collector_id, 
    (SELECT MAX(received) FROM timeseries WHERE collector_id = ts.collector_id) AS last_seen 
    FROM timeseries ts";
    Json(sqlx::query_as::<_, Collector>(SQL)
        .fetch_all(&pool)
        .await
        .unwrap())
}
```

> Don't forget to add the route! `.route("/api/collectors", get(api::show_collectors))`

### Listing Data Points for a Collector

Let's list the data points for a collector:

```rust
pub async fn collector_data(Extension(pool): Extension<sqlx::SqlitePool>, uuid: Path<String>) -> Json<Vec<DataPoint>> {
    let rows = sqlx::query_as::<_, DataPoint>("SELECT * FROM timeseries WHERE collector_id = ? ORDER BY received")
        .bind(uuid.as_str())
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(rows)
}
```

We've used the Axum extractor *path* to get the collector ID from the URL. We've also used a parameterized query to avoid SQL injection.

Add the route: `.route("/api/collector/:uuid", get(api::collector_data))`

Run the server now, and you can go to an URL like this (copy the UUID from the collectors list):

http://localhost:3000/api/collector/a96331b8-5604-4f45-9217-8e797c5ce9ea

You'll see the data points for that collector.