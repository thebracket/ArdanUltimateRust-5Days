# Dividing Workloads

> The code for this is in `divide_workload`, in the `code/02_threads` folder.

We can use threads to divide up a workload. Let's say we have a vector of numbers, and we want to add them all up. We can divide the vector into chunks, and have each thread add up its own chunk. Then we can add up the results from each thread.

```rust
fn main() {
    const N_THREADS: usize = 8;

    let to_add: Vec<u32> = (0..5000).collect(); // Shorthand for building a vector [0,1,2 .. 4999]
    let mut thread_handles = Vec::new();
    let chunks = to_add.chunks(N_THREADS);

    // Notice that each chunk is a *slice* - a reference - to part of the array.    
    for chunk in chunks {
        // So we *move* the chunk into its own vector, taking ownership and
        // passing that ownership to the thread. This adds a `memcpy` call
        // to your code, but avoids ownership issues.
        let my_chunk = chunk.to_owned();

        // Each thread sums its own chunk. You could use .sum() for this!
        thread_handles.push(std::thread::spawn(move || {
            let mut sum = 0;
            for i in my_chunk {
                sum += i;
            }
            sum
        }));
    }

    // Sum the sums from each thread.
    let mut sum = 0;
    for handle in thread_handles {
        sum += handle.join().unwrap();
    }
    println!("Sum is {sum}");
}
```

There's a lot to unpack here, so I've added comments:

1. We use a constant to define how many threads we want to use. This is a good idea, because it makes it easy to change the number of threads later. We'll use 8 threads, because my laptop happens to have 8 cores.
2. We create a vector of numbers to add up. We use the `collect` function to build a vector from an iterator. We'll cover iterators later, but for now, just know that `collect` builds a vector from a range. This is a handy shorthand for turning any range into a vector.
3. We create a vector of thread handles. We'll use this to join the threads later.
4. We use the `chunks` function to divide the vector into chunks. This returns an iterator, so we can use it in a `for` loop. Chunks aren't guaranteed to be of equal size, but they're guaranteed to be as close to equal as possible. The last chunk will be smaller than the others.
5. Now we hit a problem:
    * `chunks` is a vector owned by the main thread.
    * Each chunk is a slice --- a borrowed reference --- to part of the vector.
    * We can't pass a borrowed reference to a thread, because the thread might outlive the main thread. There's no guarantee that the order of execution will ensure that the data is destroyed in a safe order.
    * Instead, we use `to_owned` which creates an owned copy of each chunk. This is a `memcpy` operation, so it's not free, but it's safe.

This is a common pattern when working with threads. You'll often need to move data into the thread, rather than passing references.

Moving chunks like this works fine, but if you are using threads to divide up a heavy workload with a single answer --- there's an easier way!