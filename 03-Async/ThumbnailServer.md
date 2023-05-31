# Let's Build a Thumbnail Server

> The last class in this series will build a fully functional server with all the bells and whistles.

We've covered a lot of ground in the classes so far:

* We've learned to make basic Rust programs.
* We can serialize and de-serialize data.
* We've learned all about system threads, and using Rayon to make them easy to use.
* We've covered async/await for high-performance servers.
* We've talked a lot about Tokio and what it can do for you.
* We've connected to databases.
* We've built a mini web server using Axum.

That's a lot of ground to cover in just a few hours. To help it "click", let's build a server that draws together some of these concepts. We're squeezing this into the end of the class, so it will be a bit bare-bones.

## The Design Idea

We want to create a simple web server that displays thumbnails of images. It will need the following endpoints:
* `/` - Display thumbnails of all images. Includes a form for adding an image.
* `/images` - JSON list of all uploaded images.
* (post) - `/upload` - Upload a new image and create a thumbnail.
* `/image/<id>` - Display a single image.
* `/thumb/<id>` - Display a single thumbnail.
* (post) `/search` - find images by tag.

> The code for this is in `03_async/thumbnail_server`.

## Add Dependencies

We're going to be pulling together much of what we've already learned, so we have quite a few dependencies:

```bash
cargo add tokio -F full
cargo add serde -F derive
cargo add axum -F multipart
cargo add sqlx -F runtime-tokio-native-tls -F sqlite
cargo add anyhow
cargo add dotenv
cargo add futures
cargo add dotenv
cargo add tokio_util -F io
cargo add image
```

## Create the Database

Create a `.env` file in your project containing:

```
DATABASE_URL="sqlite:images.db"
```

Then create the database:

```bash
sqlx database create
```

Let's also create a migration to make our initial database:

```bash
sqlx migrate add initial
```

A file has appeared in the `migrations` directory. Let's flesh out a minimal images database:

```sql
-- Create images table
CREATE TABLE IF NOT EXISTS images
(
    id          INTEGER PRIMARY KEY NOT NULL,
    tags        TEXT                NOT NULL
);
```

Now we'll build our `main.rs` file to run with Tokio, read the `.env` file, connect to the database and run any migrations:

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read the .env file and obtain the database URL
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;

    // Get a database connection pool
    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    // Run Migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(())
}
```

This is a good time to test that everything is working. We can run the server with `cargo run` and see that it compiles and runs.

## Setup Axum with a Layer to Offer Dependency Injection for the Database

Axum can help with global state and dependency injection. We'll use this to inject the database connection pool into our handlers.

First, let's create the Axum application:

```rust
// Build Axum with an "extension" to hold the database connection pool
let app = Router::new()
    .route("/", get(test))
    .layer(Extension(pool));
let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
```

We'll also make a handler named `test` that just returns a string:

```rust
async fn test(Extension(pool): Extension<sqlx::SqlitePool>) -> String {
    let result = sqlx::query("SELECT COUNT(id) FROM images")
        .fetch_one(&pool)
        .await
        .unwrap();
    let count = result.get::<i64, _>(0);
    format!("{count} images in the database")
}
```

Now run the program. Go to `http://localhost:3000` and you will see "0 images in the database". We've injected our database pool, successfully queries the database and returned dynamic data.

## Create a Basic Homepage

Let's create the beginnings of a web page that will display our thumbnails. Create a new file named `index.html` in your `src` directory:

```html
<!DOCTYPE html>
<html>
<head>
    <title>My Awesome Thumbnail Server</title>
</head>
<body>
    <h1>Welcome to the thumbnail server</h1>
    <div id="thumbnails"></div>
    <hr />
    <h2>Add an Image</h2>
    <form method="post" action="/upload" enctype="multipart/form-data">
        <input type="text" name="tags" value="" placeholder="Tags" /> <br />
        <input type="file" name="image" /> <br />
        <input type="submit" value="Upload New Image" />
    </form>
</body>
</html>
```

Now comment out the `test` function, and add a replacement for loading the index page from disk:

```rust
async fn index_page() -> Html<String> {
    let path = Path::new("src/index.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}
```

Adjust the route to point to the new page: `.route("/", get(index_page))`.

Run the program now (`cargo run`) and you can see the HTML form. We're making progress!

## Uploading Images

Axum has great support for forms built-in, but files are always a little more complicated. The easiest way is to use a `multipart` form, which we've done in the HTML file. (You can normally use the `Form` type to automatically deserialize forms; we'll do that in the search system).

Create a new handler named `uploader`:

```rust
async fn uploader(mut multipart: Multipart) -> String {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("{name} is {} bytes", data.len());
    }
    "Ok".to_string()
}
```

And add it to your routes:

```rust
.route("/upload", post(uploader))
```

Now run the program and submit an image and some tags. You'll see something like this on the server console:

```
tags is 11 bytes
image is 1143067 bytes
```

The data made it! Now we need to turn it into useful data. Let's extract the fields we want:

```rust
async fn uploader(mut multipart: Multipart) -> String {
    let mut tags = None; // "None" means "no tags yet"
    let mut image = None;
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        match name.as_str() {
            "tags" => tags = Some(String::from_utf8(data.to_vec()).unwrap()), // Using Some means we can check we received it
            "image" => image = Some(data.to_vec()),
            _ => panic!("Unknown field: {name}"),
        }
    }

    if let (Some(tags), Some(image)) = (tags, image) { // Destructuring both Options at once

    } else {
        panic!("Missing field");
    }

    "Ok".to_string()
}
```

This gives a relatively robust extractor---we can be sure that we've received both fields we need, and will throw an error if we haven't.

You can run the program now and resubmit the form---if it doesn't error out, it's all good.

## Saving the Image

The first thing to do is to add the image with tags to the database, and obtain the new primary key. Let's create a function to do this:

```rust
async fn insert_image_into_database(pool: &Pool<Sqlite>, tags: &str) -> anyhow::Result<i64> {
    let row = sqlx::query("INSERT INTO images (tags) VALUES (?) RETURNING id")
        .bind(tags)
        .fetch_one(pool)
        .await?;

    Ok(row.get(0))
}
```

The function simply inserts the tags and returns the new id. We've used `anyhow` to simplify error handling.

Now let's call it:

```rust
if let (Some(tags), Some(image)) = (tags, image) {
    let new_image_id = insert_image_into_database(&pool, &tags).await.unwrap();
} else {
    panic!("Missing field");
}
```

We need to save the image to disk. Let's create a function to do this:

```rust
async fn save_image(id: i64, bytes: &[u8]) -> anyhow::Result<()> {
    // Check that the images folder exists and is a directory
    // If it doesn't, create it.
    let base_path = Path::new("images");
    if !base_path.exists() || !base_path.is_dir() {
        tokio::fs::create_dir_all(base_path).await?;
    }

    // Use "join" to create a path to the image file. Join is platform aware,
    // it will handle the differences between Windows and Linux.
    let image_path = base_path.join(format!("{id}.jpg"));
    if image_path.exists() {
        // The file exists. That shouldn't happen.
        anyhow::bail!("File already exists");
    }

    // Write the image to the file
    tokio::fs::write(image_path, bytes).await?;
    Ok(())
}
```

And let's call it from the uploader:

```rust
if let (Some(tags), Some(image)) = (tags, image) {
    let new_image_id = insert_image_into_database(&pool, &tags).await.unwrap();
    save_image(new_image_id, &image).await.unwrap();
} else {
    panic!("Missing field");
}
```

We're not making any thumbnails yet, but we should test our progress so far. Run the program, upload an image and check that it appears in the `images` folder.

## Displaying all the Images

Now that we have an image in both the database and the filesystem, let's display it. We'll need to create a new handler:

```rust
async fn get_image(Path(id): Path<i64>) -> impl IntoResponse {
    let filename = format!("images/{id}.jpg");
    let attachment = format!("filename={filename}");
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("image/jpeg"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        header::HeaderValue::from_str(&attachment).unwrap()
    );
    let file = tokio::fs::File::open(&filename).await.unwrap();
    axum::response::Response::builder()
        .header(header::CONTENT_TYPE, header::HeaderValue::from_static("image/jpeg"))
        .header(header::CONTENT_DISPOSITION, header::HeaderValue::from_str(&attachment).unwrap())
        .body(StreamBody::new(ReaderStream::new(file)))
        .unwrap()
}
```

This is a bit boilerplatey, but it shows you the options you have. In this case, we build the appropriate HTTP headers and use `StreamBody` to stream the contents of the image file to the client.

The `Path(id)` is a very handy Axum extractor. You can specify placeholders in the URL, and use `Path` to fill variables.

We'll also need to add the route:

```rust
.route("/image/:id", get(get_image))
```

Now run the program and go to `http://localhost:3000/image/1` to see the image we just uploaded.

## Making a thumbnail

The Rust `image` crate makes creating thumbnails easy. It also uses `Rayon` under the hood, so it's fast. Let's create a function to make a thumbnail:

```rust
fn make_thumbnail(id: i64) -> anyhow::Result<()> {
    let image_path = format!("images/{id}.jpg");
    let thumbnail_path = format!("images/{id}_thumb.jpg");
    let image_bytes: Vec<u8> = std::fs::read(image_path)?;
    let image = if let Ok(format) = image::guess_format(&image_bytes) {
        image::load_from_memory_with_format(&image_bytes, format)?
    } else {
        image::load_from_memory(&image_bytes)?
    };
    let thumbnail = image.thumbnail(100, 100);
    thumbnail.save(thumbnail_path)?;
    Ok(())
}
```

We're doing a little dance here and not trusting that the file uploaded is actually a good jpeg. I kept uploading a PNG by mistake. So we load the image as bytes, and then use `guess_format` to let `Image` figure it out for us!

Since our first image doesn't have a thumbnail yet, let's use it as test data. We'll build a function that grabs a list of all of the images in our database, checks to see if a thumbnail exists and makes one if it doesn't:

```rust
async fn fill_missing_thumbnails(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    let mut rows = sqlx::query("SELECT id FROM images")
        .fetch(pool);

    while let Some(row) = rows.try_next().await? {
        let id = row.get::<i64, _>(0);
        let thumbnail_path = format!("images/{id}_thumb.jpg");
        if !std::path::Path::new(&thumbnail_path).exists() {
            spawn_blocking(move || {
                make_thumbnail(id)
            }).await??;
        }
    }

    Ok(())
}
```

Now let's add this to the beginning of the program (before we start Axum):

```rust
// Check thumbnails
    fill_missing_thumbnails(&pool).await?;
```

> You could easily run this in the background by spawning it separately. You could also not `await` on the `spawn_blocking` call to have the background threads all run concurrently.

Now run the program and check that the thumbnail exists.

Finally, let's also make the thumbnail on upload:

```rust
if let (Some(tags), Some(image)) = (tags, image) {
    let new_image_id = insert_image_into_database(&pool, &tags).await.unwrap();
    save_image(new_image_id, &image).await.unwrap();
    spawn_blocking(move || {
        make_thumbnail(new_image_id).unwrap();
    });
} else {
    panic!("Missing field");
}
```

## Listing Available Images

Let's be lazy since it's the end of the day and copy/paste `get_image` and make a thumbnail fetching version.

```rust
async fn get_thumbnail(Path(id): Path<i64>) -> impl IntoResponse {
    let filename = format!("images/{id}_thumb.jpg");
```

(The rest of the function is unchanged)

Now let's add the route:

```rust
.route("/thumb/:id", get(get_thumbnail))
```

You can test this if you like. Go to `http://localhost:3000/thumb/1` and you should see the thumbnail.

Now let's build a JSON service that returns all of the stored images:

```rust
#[derive(Deserialize, Serialize, FromRow, Debug)]
struct ImageRecord {
    id: i64,
    tags: String,
}

async fn list_images(Extension(pool): Extension<sqlx::SqlitePool>) -> Json<Vec<ImageRecord>> {
    sqlx::query_as::<_, ImageRecord>("SELECT id, tags FROM images ORDER BY id")
        .fetch_all(&pool)
        .await
        .unwrap()
        .into()
}
```

And of course, we add a route for it:

```rust
.route("/images", get(list_images))
```

Let's test that real quick. Go to `http://localhost:3000/images` and you should see a JSON list of all of the images.

## Client Side Thumbnail Display

Now let's modify the HTML. You can leave the server running for live testing. We need to call the `/images` endpoint and display the thumbnails.

```html
<script>
    async function getImages() {
        const response = await fetch('/images');
        const images = await response.json();

        let html = "";
        for (let i=0; i<images.length; i++) {
            html += "<div>" + images[i].tags + "<br />";
            html += "<a href='/image/" + images[i].id + "'>";
            html += "<img src='/thumb/" + images[i].id + "' />";
            html += "</a></div>";
            
        }
        document.getElementById("thumbnails").innerHTML = html;
    }

    getImages();
</script>
```

## Redirect on POST

Rather than just saying "Ok" when someone uploads an image, let's redirect back to the list of images.

We'll make a simple file, `src/redirect.html`:

```html
<html>
    <body>
        Image Uploaded!

        <script>
            function redirect() {
                window.location.href="/";
            }
            setTimeout(redirect, 1000);
        </script>
    </body>
</html>
```

Now change the `uploader` function signature to return `Html<String>`:

```rust
async fn uploader(
    Extension(pool): Extension<sqlx::SqlitePool>,
    mut multipart: Multipart,
) -> Html<String> {
```

And change the bottom to load `redirect.html` and return it as HTML:

```rust
    let path = std::path::Path::new("src/redirect.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}
```

Now run the program and add a file. You should go back to the list of images, with your new image displayed.

## Add Search

Finally, I promised a search function.

Let's start by adding another form to `index.html`:

```html
<body>
    <h1>Welcome to the thumbnail server</h1>
    <div id="thumbnails"></div>
    <hr />
    <form method="post" action="/search">
        <input type="text" name="tags" value="" placeholder="Tags" /> <br />
        <input type="submit" value="Search" />
    </form>
    <hr />
    <h2>Add an Image</h2>
```

Next, we create a Rust structure to receive the contents of the form post:

```rust
#[derive(Deserialize)]
struct Search {
    tags: String
}
```

Now we can build a handler to server-side render the results:

```rust
async fn search_images(Extension(pool): Extension<sqlx::SqlitePool>, Form(form): Form<Search>) -> Html<String> {
    let tag = format!("%{}%", form.tags);

    let rows = sqlx::query_as::<_, ImageRecord>("SELECT id, tags FROM images WHERE tags LIKE ? ORDER BY id")
        .bind(tag)
        .fetch_all(&pool)
        .await
        .unwrap();

    let mut results = String::new();
    for row in rows {
        results.push_str(&format!("<a href=\"/image/{}\"><img src='/thumb/{}' /></a><br />", row.id, row.id));
    }

    let path = std::path::Path::new("src/search.html");
    let mut content = tokio::fs::read_to_string(path).await.unwrap();
    content = content.replace("{results}", &results);

    Html(content)
}
```

It is referring to `src/search.html`, so let's make that too. Notice the `{results}` placeholder.

> Tip: There are some great placeholder libraries!

```html
<!DOCTYPE html>
<html>

<head>
    <title>My Awesome Thumbnail Server</title>
</head>

<body>
    <h1>Welcome to the thumbnail server</h1>
    <div id="thumbnails">{results}</div>
    <hr />
    <form method="post" action="/search">
        <input type="text" name="tags" value="" placeholder="Tags" /> <br />
        <input type="submit" value="Search" />
    </form>

</body>

</html>
```

And run the program and you can search!

233 lines of code in the last coding session of the day. You've:

* Setup a database with migrations.
* Auto-run the migrations on start.
* Made a thumbnail of any image that has been uploaded and doesn't have one.
* Setup a multi-part from post that saves image information to the database and saves the file.
* Called into system thread land to generate an image thumbnail.
* Added a redirect to handle a POSTed form.
* Added a simple search function.

Not bad - Rust is *very* productive once you get going.