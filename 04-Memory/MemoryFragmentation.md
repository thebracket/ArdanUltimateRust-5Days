# Memory Fragmentation

If your program is going to be running for a long time (maybe its a server), memory fragmentation can become a hard to diagnose problem. It can also be a problem if you're working with a lot of data and you're running out of memory.

The heap is allocated by the operating system. Every time you request a chunk of data, the available heap is searched for a chunk that is big enough to hold the requested memory---contiguously. If there isn't one, the heap is expanded. If the heap is expanded, the operating system has to find a new chunk of memory that is big enough to hold the new heap---contiguously. If it can't, it has to expand the heap again. This can lead to a lot of memory fragmentation.

So now imagine that you are allocating and de-allocating repeatedly---with differing size chunks. You can end up using more memory than you need to, because the heap is fragmented.

> This is the same as the old `defrag` program on disks!

Most of the time, you don't need to worry. Allocators are pretty smart and will try to avoid this problem. But, I'd like you to think about a few ways to not run into this problem:

* If you're going to be storing a large amount of data over time, consider using a `Vec`, `HashMap` or other structure with pre-allocated capacity. Don't "shrink to fit" when you clear it or remove items. Let the vector act as a big buffer, expanding within its allocated size. This will reduce the number of times the heap has to be expanded, and remove the need to find a contiguous chunk of memory for the new heap.
* If you're on a platform where expanding the heap is expensive, consider using an Arena---we'll talk about that in a bit.

## Changing Allocator

> We're only going to touch on this, it's quite advanced. It's also a useful tool to have in your kit.

By default, Rust uses the allocator that comes with your platform. `malloc` on UNIX-likes, and `HeapAlloc` on Windows. You can opt-in to using a different allocator. `JemAlloc` is a popular allocator, optimized for repeated allocation, reallocation and removal of memory. It's also optimized for multi-threaded use. You can use it by adding this to your `Cargo.toml`:

```toml
[dependencies]
jemallocator = "0.5"
```

Then in your program you can use it like this:

```rust
#[global_allocator]
static GLOBAL: Jemalloc = jemallocator::Jemalloc;;
```

Windows isn't a great choice for the Jem allocator---but it's a *great* choice for Linux and Mac. It carries some downsides:

* It's not as well tested as the default allocator---but it's very stable in practice. FreeBSD still uses it, Rust used to use it by default, and lots of projects use it. My own `LibreQoS` used it in the main monitor daemon, and it's been both very stable and very fast. (LibreQoS tracks a LOT of statistics, causing quite a bit of memory churn.)
* Some tools such as `valgrind` assume that you are going to be using the default allocator. It won't give you as much useful information if you use it on your program.

If you check out the [jemalloc website](https://jemalloc.net/) there are a lot of really helpful tools included. You can instrument memory allocation and build detailed reports as to what you are doing. It's sometimes worth switching to Jemalloc just to run the tools and see what's going on---and then switch back if needs-be.

See [this gist](https://gist.github.com/ordian/928dc2bd45022cddd547528f64db9174) for a guide to using Jemalloc for heap profiling.

## Arenas

An "Arena" is a pre-allocated area of memory that you use over and over to store data. A vector with a preset capacity that will never grow is the simplest form of arena. Arenas are used:

* When you need to avoid memory fragmentation.
* When allocation is expensive; you allocate your arena up-front and then use it.
* When you absolutely can't afford to fail to allocate memory. Arenas are pre-allocated, so you can't fail to allocate memory once they are started.
* On some platforms (real-time), you can't allocate memory after the program starts. You have to pre-allocate everything you need.
* Some arenas are used to store data that is never de-allocated. This is common in games, where you might have a "level" arena, a "game" arena, and a "global" arena. You can de-allocate the level arena when you move to a new level, but the game and global arenas are never de-allocated.
* In turn, this can allow you to fix the pain of "cyclic references"---references that refer to one another. If you have a cyclic reference, you can't de-allocate the memory. If you use an arena, you can de-allocate the entire arena at once, and the cyclic references are no longer a problem.

### Bump Style Arenas

A "bump" style arena is the simplest form of arena. You allocate a chunk of memory, and then you just keep allocating from it. You keep a pointer to the end of the allocated memory, and when you run out, you allocate another chunk. You can't de-allocate memory, but you can de-allocate the entire arena at once.

This allows you to solve cyclic references, and by pre-allocating the arena, you can avoid the problem of running out of memory.

> See `code/04_mem/arena_bump` for code.

We'll test out [Bumpalo](https://docs.rs/bumpalo/latest/bumpalo/). Bumpalo is pretty easy to use:

```rust
use bumpalo::Bump;

struct MyData {
    a: i32,
}

fn main() {
    let arena = Bump::new();
    arena.set_allocation_limit(Some(8192)); // Limit the size of the arena to 8 KiB
    let x = arena.alloc(MyData { a: 123 });
}
```

You can also enable the `collections` feature and use `BumpaloVec` and `BumpaloString` to store data in the arena:

```rust
use bumpalo::Bump;
use bumpalo::collections::String;
use bumpalo::collections::Vec;

struct MyData {
    a: i32,
}

fn main() {
    let arena = Bump::new();
    arena.set_allocation_limit(Some(8192)); // Limit the size of the arena to 8 KiB
    let x = arena.alloc(MyData { a: 123 });

    // With collections enabled
    let mut my_string = String::from_str_in("Hello, world!", &arena);
    let mut vec = Vec::new_in(&arena);
    vec.push(12);
}
```

**Downside**: `Drop` will never be called in a `Bump` arena. You can enable unstable compiler features and make it work, but for now---you're not dropping anything in the arena!

Use a `bump` arena to allocate memory up-front (or in chunks) and store data inside the arena. You can't de-allocate individual items, but for something like a data-collector that *must not* suddenly fail to allocate memory or expand its heap, it's a great choice.

## Slab Arenas

A "slab arena" pre-allocates space for a uniform type, indexing each entry by key. This is similar to a pre-allocated `Vec`, but you don't have to keep `usize` around for entries---and the slab keeps track of vacant entries for you. It's also similar to a `HashMap`, but you don't have to hash keys. Slabs are great for pre-allocating a big pool of resources and then using them as needed.

> See `code/04_mem/arena_slab` for code.

```rust
use slab::Slab;

fn main() {
    let mut slab = Slab::with_capacity(10);
    let hello = slab.insert("hello");
    let world = slab.insert("world");

    assert_eq!(slab[hello], "hello");
    assert_eq!(slab[world], "world");

    slab.remove(hello);
}
```

Note that you *can* remove items! Slabs work like a "slot map" - entries are either `Vacant` or filled with your data type. Slabs won't ever fragment, and entries will be stored in contiguous memory. This makes them very fast to iterate over. If you can preallocate a slab of data, it's a great choice for high-performance and not fragmenting memory.
