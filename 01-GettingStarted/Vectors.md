# Vectors

Vectors are like arrays, but can change size. They are stored on the heap, so they are slower than arrays, but they are still fast. They are the most common collection type in Rust. Vectors guarantee that everything will be stored contiguously. They also allocate spare space - so you are using more memory than you need, but you don't have to reallocate as often. Vectors double in size every time you run out of capacity---which makes them fast, but you can waste a lot of memory if you don't need it.

> The code for this example is in `auth_vec` and `login_vec`.

Let's create a vector of users:

```rust
pub fn get_users() -> Vec<User> {
    vec![
        User::new("admin", "password", Role::Admin),
        User::new("bob", "password", Role::User),
    ]
}
```

The `vec!` macro is a helper that moves a list of entries in array format into a vector. You can also do:

```rust
pub fn get_users() -> Vec<User> {
    let mut users = Vec::new();
    users.push(User::new("admin", "password", Role::Admin));
    users.push(User::new("bob", "password", Role::User));
    users
}
```

Now the great part is that the `login` function doesn't need to change. Iterators are standardized across most collection types, so you can use the same code for arrays and vectors.

## Vector Growth

> Tip: if you know how big a vector should be, you can create it with `Vec::with_capacity(n)` to avoid reallocation.

> The code for this section is in `vector_growth`.

Let's create a quick side project to see how vectors grow. Create a new project with `cargo new vector_growth`. Don't forget to update the workspace! Add this to `src/main.rs`:

```rust
fn main() {
    let mut my_vector = Vec::new();
    for _ in 0..20 {
        my_vector.push(0);
        println!("Size: {}, Capacity: {}", my_vector.len(), my_vector.capacity());
    }
}
```

This shows:

```
Size: 1, Capacity: 4
Size: 2, Capacity: 4
Size: 3, Capacity: 4
Size: 4, Capacity: 4
Size: 5, Capacity: 8
Size: 6, Capacity: 8
Size: 7, Capacity: 8
Size: 8, Capacity: 8
Size: 9, Capacity: 16
Size: 10, Capacity: 16
Size: 11, Capacity: 16
Size: 12, Capacity: 16
Size: 13, Capacity: 16
Size: 14, Capacity: 16
Size: 15, Capacity: 16
Size: 16, Capacity: 16
Size: 17, Capacity: 32
Size: 18, Capacity: 32
Size: 19, Capacity: 32
Size: 20, Capacity: 32
```

Now imagine that you are downloading 1,000,000 items from a database. You want to be careful that you aren't using 2,000,000 capacity slots when you only need 1,000,000. You can use `Vec::shrink_to_fit()` to reduce the capacity to the size of the vector. This is a hint to the compiler, so it may not actually shrink the vector. You can also use `Vec::reserve(n)` to reserve `n` slots in the vector. This is a hint to the compiler, so it may not actually reserve the slots.

## Collecting from Iterators

You can collect from an iterator into a vector. This is useful if you want to filter or map a vector. For example, let's say that you want to get all of the users with a role of `User`. You can do this:

```rust
let users: Vec<User> = get_users().into_iter().filter(|u| u.role == Role::User).collect();
```

## Deleting from a Vector---using Retain

You can delete vector entries with retain. This will delete all users except for "kent". Retain takes a function---closure---that returns true if an entry should be kept.

```rust
users.retain(|u| u.username == "kent");
```

## Deleting from a Vector---using Remove

You can delete vector entries with remove. This will delete the first user. Remove takes an index.

```rust
users.remove(0);
```

## Deleting from a Vector---using Drain

Drain is a special type of delete. It will delete everything, and give it to you as an iterator on the way out. This is useful if you want to delete everything, but you want to do something with the data before you delete it.

```rust
let deleted_users: Vec<User> = users.drain(..).collect();
```

Or more usefully:

```rust
let deleted_users: Vec<User> = users.drain(..).for_each(|user| println!("Deleting {user:?}"));
```

Vectors really are a swiss-army knife: they can do almost anything. They are fast, and they are easy to use. They are the most common collection type in Rust.