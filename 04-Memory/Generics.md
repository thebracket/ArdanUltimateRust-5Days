# Generics

Generics are very closely tied to traits. "Generics" are meta-programming: a way to write "generic" code that works for multiple types. Traits are a way to specify the requirements for a generic type.

The simplest generic is a function that takes a generic type. Who'se sick of typing `to_string()` all the time? I am! You can write a generic function that accepts any type that implements `ToString`---even `&str` (bare strings) implement `ToString`:

```rust
fn print_it<T: ToString>(x: T) {
    println!("{}", x.to_string());
}
```

So now you can call `print_it` with `print_it("Hello")`, `print_it(my_string)` or even `print_it(42)` (because integers implement `ToString`).

There's a second format for generics that's a bit longer but more readable when you start piling on the requirements:

```rust
fn print_it<T>(x: T)
where
    T: ToString,
{
    println!("{}", x.to_string());
}
```

You can combine requirements with `+`:

```rust
fn print_it<T>(x: T)
where
    T: ToString + Debug,
{
    println!("{:?}", x);
    println!("{}", x.to_string());
}
```

You can have multiple generic types:

```rust
fn print_it<T, U>(x: T, y: U)
where
    T: ToString + Debug,
    U: ToString + Debug,
{
    println!("{:?}", x);
    println!("{}", x.to_string());
    println!("{:?}", y);
    println!("{}", y.to_string());
}
```

The generics system is almost a programming language in and of itself---you really can build most things with it.

## Traits with Generics

> See the `04_mem/trait_generic` project.

Some traits use generics in their implementation. The `From` trait is particularly useful, so let's take a look at it:

```rust
struct Degrees(f32);
struct Radians(f32);

impl From<Radians> for Degrees {
    fn from(rad: Radians) -> Self {
        Degrees(rad.0 * 180.0 / std::f32::consts::PI)
    }
}

impl From<Degrees> for Radians {
    fn from(deg: Degrees) -> Self {
        Radians(deg.0 * std::f32::consts::PI / 180.0)
    }
}
```

Here we've defined a type for Degrees, and a type for Radians. Then we've implemented `From` for each of them, allowing them to be converted from the other. This is a very common pattern in Rust. `From` is also one of the few surprises in `Rust`, because it *also* implements `Into` for you. So you can use any of the following:

```rust
let behind_you = Degrees(180.0);
let behind_you_radians = Radians::from(behind_you);
let behind_you_radians2: Radians = Degrees(180.0).into();
```

You can even define a function that requires that an argument be convertible to a type:

```rust
fn sin(angle: impl Into<Radians>) -> f32 {
    let angle: Radians = angle.into();
    angle.0.sin()
}
```

And you've just made it impossible to accidentally use degrees for a calculation that requires Radians. This is called a "new type" pattern, and it's a great way to add constraints to prevent bugs.

You can *also* make the `sin` function with generics:

```rust
fn sin<T: Into<Radians>>(angle: T) -> f32 {
    let angle: Radians = angle.into();
    angle.0.sin()
}
```

The `impl` syntax is a bit newer, so you'll see the generic syntax more often.

## Generics and Structs

You can make generic structs and enums, too. In fact, you've seen lots of generic `enum` types already: `Option<T>`, `Result<T, E>`. You've seen plenty of generic structs, too: `Vec<T>`, `HashMap<K,V>` etc.

Let's build a useful example. How often have you wanted to add entries to a `HashMap`, and instead of replacing whatever was there, you wanted to keep a list of *all* of the provided values that match a key.

> The code for this is in `04_mem/hashmap_bucket`.

Let's start by defining the basic type:

```rust
use std::collections::HashMap;

struct HashMapBucket<K,V>
{
    map: HashMap<K, Vec<V>>
}
```

The type contains a `HashMap`, each key (of type `K`) referencing a vector of values (of type `V`). Let's make a constructor:

```rust
impl <K,V> HashMapBucket<K,V> 
{
    fn new() -> Self {
        HashMapBucket {
            map: HashMap::new()
        }
    }
}

So far, so good. Let's add an `insert` function (inside the implementation block):

```rust
fn insert(&mut self, key: K, value: V) {
    let values = self.map.entry(key).or_insert(Vec::new());
    values.push(value);
}
```

Uh oh, that shows us an error. Fortunately, the error tells us exactly what to do---the key has to support `Eq` (for comparison) and `Hash` (for hashing). Let's add those requirements to the struct:

```rust
impl <K,V> HashMapBucket<K,V> 
where K: Eq + std::hash::Hash
{
    fn new() -> Self {
        HashMapBucket {
            map: HashMap::new()
        }
    }

    fn insert(&mut self, key: K, value: V) {
        let values = self.map.entry(key).or_insert(Vec::new());
        values.push(value);
    }
}
```

So now we can insert into the map and print the results:

```rust
fn main() {
    let mut my_buckets = HashMapBucket::new();
    my_buckets.insert("hello", 1);
    my_buckets.insert("hello", 2);
    my_buckets.insert("goodbye", 3);
    println!("{:#?}", my_buckets.map);
}
```

In 21 lines of code, you've implemented a type that can store multiple values for a single key. That's pretty cool. Generics are a little tricky to get used to, but they can really supercharge your productivity.

## Amazing Complexity

If you look at the `Bevy` game engine, or the `Axum` webserver, you'll find the most mind-boggling combinations of generics and traits. It's not uncommon to see a type that looks like this:

Remember how in Axum you could do dependency injection by adding a layer containing a connection pool, and then every route could magically obtain one by supporting it as a parameter? That's generics and traits at work.

In both cases:
* A function accepts a type that meets certain criteria. Axum layers are cloneable, and can be sent between threads.
* The function stores the layers as a generic type.
* Routes are also generic, and parameters match against a generic+trait requirement. The route is then stored as a generic function pointer.

There's even code that handles `<T1>`, `<T1, T2>` and other lists of parameters (up to 16) with separate implementations to handle whatever you may have put in there!

It's beyond the scope of a foundations class to really dig into how that works---but you have the fundamentals. 