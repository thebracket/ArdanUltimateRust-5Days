# Iterators

You've used iterators---and their async cousin streams---a lot:

* Calling `iter` has created an iterator over a collection. The iterator returns *borrowed references* to each element of the collection.
* Calling `iter_mut` does the same, but gains a mutable reference to each element.
* Calling `into_iter` does the same, but gains ownership of each element (returning it out of the collection).
* `Drain` is a special iterator that returns owned elements from a collection, but also removes them from the collection---leaving the collection empty but usable.

Once you have an iterator, you have a lot of iterator functions---map, filter, reduce, fold, etc.

## Memory and Iterators

When you use `iter()` and `iter_mut()`, you aren't having much memory impact: a single pointer to each element is created. Using `into_iter()` generates a move for each item (which is sometimes optimized away). So in general, iterators are a very lightweight way to operate on a collection.

As soon as you `collect`, you are using memory for every collected item. If you're collecting references, that's one pointer per collected item. If you're cloning---you just doubled your memory usage. So be careful!

## Creating Your Own Iterator

> The code for this is in `code/04_mem/iterator_generator`.

An iterator is a type that implements the `Iterator` trait. The trait has one function: `next()`. It returns an `Option<T>` where `T` is the type of the iterator's elements. If the iterator is done, it returns `None`. Otherwise, it returns `Some(element)`. The type itself can store any state required for the its task.

You don't have to iterate *over* anything---you can use an iterator as a generator. Let's make a simple iterator that counts up from 0 to a "max" number. We'll start with a simple structure and constructor:

```rust
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}
```

We can implement `Iterator` for `Counter`:

```rust
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```

So the iterator adds one to its stored value and returns it. That will give you a sequence of numbers from 1 to `max`. We can use it like this:

```rust
fn main() {
    let numbers: Vec<u32> = Counter::new(10).collect();
    println!("{numbers:?}");
}
```

This will print `[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]`.

If you'd rather return and then add, you can tweak the iterator:

```rust
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            let result = Some(self.count);
            self.count += 1;
            result
        } else {
            None
        }
    }
}
```

This will print `[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]`.

### Optimization---Exact Size Iterators

You can achieve a small speed improvement by *also* indicating that the iterator returns a fixed size:

```rust
impl ExactSizeIterator for Counter {
    fn len(&self) -> usize {
        self.max as usize
    }
}
```

Knowing that he iterator will always return a fixed size allows the compiler to perform some optimizations.

To really optimize it, you need can set `MAX` as a compile-time constant. This gets a bit messy (there's compile-time generic markers everywhere)---but it works. This is a good example of code that belongs in a library:

```rust
struct Counter<const MAX: u32> {
    count: u32,
}

impl <const MAX:u32> Counter<MAX> {
    fn new() -> Self {
        Self { count: 0 }
    }
}

impl <const MAX:u32> Iterator for Counter<MAX> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < MAX {
            let result = Some(self.count);
            self.count += 1;
            result
        } else {
            None
        }
    }
}

impl <const MAX:u32> ExactSizeIterator for Counter<MAX> {
    fn len(&self) -> usize {
        MAX as usize
    }
}

fn main() {
    let numbers: Vec<u32> = Counter::<10>::new().collect();
    println!("{numbers:?}");
}
```

This helps because the optimizer can see exactly how many iterations will occur, and can reduce the number of bounds-checking calls that are required.

## Iterating Over a Collection

> The code for this is in `code/04_mem/iterator_hashbucket`.

Remember [we created a `HashMapBucket` type](./Generics.md)? Let's extend it to provide an iterator.

We'll start by making an empty iterator type:

```rust
struct HashMapBucketIter;

impl Iterator for HashMapBucketIter {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
```

Now, let's add some generic properties---including a lifetime. You have to be sure that the structure you are iterating over will last longer than the iterator itself:

```rust
struct HashMapBucketIter<'a, K, V> {
    key_iter: std::collections::hash_map::Iter<'a, K, Vec<V>>,
    current_map_entry: Option<(&'a K, &'a Vec<V>)>,
    current_vec_index: usize,
}
```

You've added:
* `key_iter` - an iterator over the map itself. This will return a reference to each key, and a reference to the vector of values. It's exactly the same as calling `iter()` on the stored map.
* `current_map_entry` - the current key and vector of values. This is an `Option` because the iterator might be complete. It's the same as calling `next()` on `key_iter`.
* `current_vec_index`. Since we're returning each (K,V) entry separately, we need to store our progress through each key's iterator.

Now we can add an `iter()` function to the `HashMapBucket` itself:

```rust
impl <K,V> HashMapBucket<K, V> {
    fn iter(&self) -> HashMapBucketIter<K, V> {
        let mut key_iter = self.map.iter();
        let current_map_entry = key_iter.next();
        HashMapBucketIter {
            key_iter,
            current_map_entry,
            current_vec_index: 0,
        }
    }
}
```

See how this aligns with the iterator type we created? We create an iterator over the map and store it in the iterator. This is why you had to specify a lifetime: you have to convince Rust that the iterator MUST out-live the map itself. Then we call `next` on the iterator to obtain the first key and vector of values (a reference - we're not copying or cloning).

With this data, we can build the iterator itself:

```rust
// Specify 'a - the lifetime, and K,V on both sides.
// If you wanted to change how the iterator acts for a given type of key or
// value you could cange the left-hand side.
impl <'a, K, V> Iterator for HashMapBucketIter<'a, K, V> {
    // Define `Item` - a type used inside the trait - to be a reference to a key and value.
    // This specifies the type that the iterator will return.
    type Item = (&'a K, &'a V);

    // You use Item to specify the type returned by `Next`. It's always an option of the type.
    fn next(&mut self) -> Option<Self::Item> {
        // If there is a current map entry, and a current vec index
        if let Some((key, values)) = self.current_map_entry {
            // If the index is less than the length of the vector
            if self.current_vec_index < values.len() {
                // Get the value at the current index
                let value = &values[self.current_vec_index];
                // Increment the index
                self.current_vec_index += 1;
                // Return the key and value
                return Some((key, value));
            } else {
                // We're past the end of the vector, next key
                self.current_map_entry = self.key_iter.next();
                self.current_vec_index = 0;

                if let Some((key, values)) = self.current_map_entry {
                    // If the index is less than the length of the vector
                    if self.current_vec_index < values.len() {
                        // Get the value at the current index
                        let value = &values[self.current_vec_index];
                        // Increment the index
                        self.current_vec_index += 1;
                        // Return the key and value
                        return Some((key, value));
                    }
                }
            }
        }

        None
    }
}
```