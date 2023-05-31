# Working With Databases

If there's one task that's common to Enterprise systems, it's waiting for the database. A lot of programs spend time waiting for the database---adding data, reading it, updating it, deleting it. Database interaction is one area in which async really shines---and Rust has tools to help with it.

## Setup

I don't want to ask everyone to install a local copy of PostgreSQL or similar just for this class, that'd be excessive. Instead, we'll use `sqlite`---a tiny self-contained database. It's not very powerful, but it gets the job done.

> The code for this example is in `03_async/database`.

Let's start by adding some crates to our program:

```bash
cargo add tokio -F full
cargo add sqlx -F runtime-tokio-native-tls -F sqlite
cargo add anyhow
cargo add dotenv
cargo add futures
```

We'll also install the `sqlx` command-line tool with:

```bash
cargo install sqlx-cli
```

Lastly, we need to tell `sqlx` where to find the database we'll be using. In the top-level of your project (next to `Cargo.toml` and the `src` directory) create a file named `.env`. This is a helper for setting environment variables.

In `.env`, add the following line:

```bash
DATABASE_URL="sqlite:hello_db.db"
```

## Create the Database

You can tell `sqlx` to create an empty database by typing:

```bash
sqlx database create
```

Notice that "hello_db.db" has appeared! This is the database file. You can open it with a SQLite client if you want to poke around.

## Create a Migration

*Migrations* are a common process in applications. You define an initial migration to build tables and add any initial data you require. Then you add migrations to update the database as your application evolves. `sqlx` supports migrations natively, and can build them into your program.

Let's create a migration.

```bash
sqlx migrate add initial
```

`initial` is just the name of the migration. If you look in the source folder, a "migrations" folder has appeared. A `.sql` file containing the migration has been created. It's largely empty.

Let's add some SQL to create a table:

```sql
-- Create a messages table
CREATE TABLE IF NOT EXISTS messages
(
    id          INTEGER PRIMARY KEY NOT NULL,
    message     TEXT                NOT NULL
);

--- Insert some test messages
INSERT INTO messages (id, message) VALUES (1, 'Hello World!');
INSERT INTO messages (id, message) VALUES (2, 'Hello Galaxy!');
INSERT INTO messages (id, message) VALUES (3, 'Hello Universe!');
```

You can run the migrations with:

```bash
sqlx migrate run
```

An extra table is created storing migration status in the database. Migrations won't be run twice.

## Accessing the Database via Async Rust

Now that we have a database, let's wire it up with some Rust.

```rust
use sqlx::Row;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read the .env file and obtain the database URL
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;

    // Get a database connection pool
    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    // Fetch the messages from the database
    let messages = sqlx::query("SELECT id, message FROM messages")
        .map(|row: sqlx::sqlite::SqliteRow| {
            let id: i64 = row.get(0);
            let message: String = row.get(1);
            (id, message)
        })
        .fetch_all(&pool)
        .await?;

    // Print the messages
    for (id, message) in messages {
        println!("{id}: {message}");
    }

    Ok(())
}
```

The program outputs the data we inserted:

```
1: Hello World!
2: Hello Galaxy!
3: Hello Universe!
```

## Let's Make this a Bit Easier

Mapping each row and parsing with `get` is messy---and you don't have to do it! Sqlx supports a `FromRow` system that can automatically convert rows into Rust structs.

Start by making a structure to hold the data:

```rust
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct Message {
    id: i64,
    message: String,
}
```

Then you can update the query to be much simpler:

```rust
let messages = sqlx::query_as::<_, Message>("SELECT id, message FROM messages")
    .fetch_all(&pool)
    .await?;

// Print the messages
for message in messages.into_iter() {
    println!("{message:?}");
}
```

> `sqlx` is NOT an ORM (Object-Relational-Manager). It won't handle updating the structure and building SQL for you. There are options including SeaOrm and Diesel for this if you need it.

## How About Streaming?

Retrieving every single record with `fetch_all` is fine for small queries, but what is you are retrieving a million records? You will potentially cause all manner of performance problems.

> Aside: If you actually need to query a million records at once, that's often a sign of an architectural issue. You should consider smaller chunks, cursors/pagination. You should really check if you actually *need* all million, or can use a filter.

We talked about streams a bit before. A stream is like an iterator, but accessing the next entry is an async operation. This has two advantages:
* You are no longer blocking while you retrieve each row.
* The database driver can receive rows one at a time, reducing overall load.

Conversely---it's not as fast, *because* you are waiting on each row.

Let's try it out:

```rust
println!("--- stream ---");
use futures::TryStreamExt;
let mut message_stream = sqlx::query_as::<_, Message>("SELECT id, message FROM messages")
    .fetch(&pool);
while let Some(message) = message_stream.try_next().await? {
    println!("{message:?}");
}
```

## Let's Automate our Migrations

Having to run the migrations tool by hand each time is cumbersome. We can automate that, too.

This is pretty straightforward. Add the following:

```rust
// Get a database connection pool
// <--> To tell you where this goes

// Run Migrations
sqlx::migrate!("./migrations")
    .run(&pool)
    .await?;
```

Now let's make another migration that adds a bit more data to the database:

```bash
sqlx migrate add more_messages
```

And we'll set the migration contents to:

```sql
INSERT INTO messages (id, message) VALUES (4, 'Another Message');
INSERT INTO messages (id, message) VALUES (5, 'Yet Another Message');
INSERT INTO messages (id, message) VALUES (6, 'Messages Never End');
```

Now *don't* run the sqlx migration command. Instead, run your program.

The migration ran, and you see your new data:

```
--- stream ---
Message { id: 1, message: "Hello World!" }
Message { id: 2, message: "Hello Galaxy!" }
Message { id: 3, message: "Hello Universe!" }
Message { id: 4, message: "Another Message" }
Message { id: 5, message: "Yet Another Message" }
Message { id: 6, message: "Messages Never End" }
```

Run it again. You don't get even more data appearing (or errors about duplicate keys). The migrations table has ensures the migration is not run twice.

## Updating Data

Running update and delete queries uses slightly different syntax, but it's basically the same. Let's update the first message:

First, we'll create a function.

```rust
async fn update_message(id: i64, message: &str, pool: &sqlx::SqlitePool) -> anyhow::Result<()> {
    sqlx::query("UPDATE messages SET message = ? WHERE id = ?")
        .bind(message)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
```

Note:

* `.bind` replaces placeholders in the query with the values you provide. This is a good way to avoid SQL injection attacks.
* `.execute` runs a query that isn't expecting an answer other than success or failure.

And then in `main` we call it:

```rust
// Update message 1
update_message(1, "First Message", &pool).await?;
```

## Let's Add Tracing

`sqlx` supports tracing, so you can see what's going on under the hood. Let's add it to our program.

Start by adding the tracing subscriber to your `Cargo.toml`:

```bash
cargo add tracing
cargo add tracing-subscriber
```

Add a subscription to the tracing to your `main` function:

```rust
// Enable tracing
tracing_subscriber::fmt::init();
```

Now run the program unchanged. You will see lots of extra information:

```
2023-05-31T15:11:57.330979Z  INFO sqlx::query: SELECT id, message FROM …; rows affected: 1, rows returned: 6, elapsed: 94.900µs

SELECT
  id,
  message
FROM
  messages
```

> If you didn't see anything, set an environment variable `RUST_LOG=info`. On *NIX, you can do `RUST_LOG=info cargo run`. On Windows, `$Env:RUST_LOG=info` sets the variable.
