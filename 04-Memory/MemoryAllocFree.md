# Memory Allocation and De-Allocation

Unless you are working on embedded, real-time or other really low-level systems, you probably won't need to manually allocate and de-allocate memory. Rust has very good memory management out-of-the-box, and you can get a long way without needing to worry about it. This section serves:

* To show you what you can do if you need it.
* To help you understand why `Box`, `Vec` and other types are so useful---and what they actually do.

## The Stack and Primitives

"Primitive" types (such as `u32`, `i8` and `usize`/`isize`---whose size is the pointer size of your platform) are natively supported by CPUs. You can store them on the stack, copy them between functions and generally not worry about things like ownership, borrowing and lifetimes. In fact, it's often *slower* to borrow a `u32` than it is to copy it. Borrowing creates a pointer, which might be 64-bits in size, whereas the `u32` itself is only 32-bits.

So when you are using primitives, you really don't have to worry. The stack will ensure that when a function ends, any variables on the stack will be cleaned up. Arrays on the stack are cleaned up, too.

The stack is small---64 kb by default on Linux. So you can't put everything in there.

## Manually Allocating & De-allocating Memory

The "heap" is a region of memory that is shared by your program, and doesn't have the size-restrictions of the stack. It is *always* allocated and de-allocated. In "managed" languages, the language runtime is still allocating to the heap---but it uses a garbage collector of some sort to de-allocate memory that is no longer needed. This has the advantage that you don't need to worry about it, and the disadvantages:

* You don't know for sure when memory will be allocated. Is it allocated up-front? That's great for systems with a fixed memory size, but not so good for systems where you want to allocate memory on-demand. Is it allocated on first use? That's great for systems where you don't know how much memory you need up-front, but not so good for systems where you want to allocate memory up-front.
* You don't know for sure when the memory will be de-allocated.
* You get the infamous "GC pauses" where the program stops for a while to do garbage collection. The pauses might be very short, but it's still an insurmountable problem if you are trying to control the braking system on a sports car!
* You often have to jump through hoops to use an *exact* heap size, causing issues on embedded systems.

On some embedded platforms, you pretty much get to start out with a `libc` implementation (that may not be complete). On others, you get a platform definition file and have to do things the hard way --- we're not going that far!

### libc_malloc example

> This is in the `code/04_mem/libc_malloc` folder.

```rust
fn allocate_memory_with_libc() {
    unsafe {
        // Allocate memory with libc (one 32-bit integer)
        let my_num: *mut i32 = libc::malloc(std::mem::size_of::<i32>() as libc::size_t) as *mut i32;
        if my_num.is_null() {
            panic!("failed to allocate memory");
        }

        // Set the allocated variable - dereference the pointer and set to 42
        *my_num = 42;
        assert_eq!(42, *my_num);

        // Free the memory with libc - this is NOT automatic
        libc::free(my_num as *mut libc::c_void);
    }
}

fn main() {
    allocate_memory_with_libc();
}
```

So if you find yourself having to use `libc`, this is what you can expect: it looks a LOT like C! In your `unsafe` block, you are calling `malloc`, checking that it gave you the memory you requested, then setting the value of the memory and finally freeing it.

If you forget to call `free`, then just like a C program---you leaked memory.

### Using Rust's Allocator

Using `malloc` isn't always as simple as it sounds, you need to worry about memory alignment (lining up memory blocks with your platform's "word size"). Rust provides an allocator setup that you can use instead. It's similar, and still `unsafe`:

```rust
fn allocate_memory_with_rust() {
    use std::alloc::{alloc, dealloc, Layout};

    unsafe {
        // Allocate memory with Rust. It's safer to force alignment.
        let layout = Layout::new::<u16>();
        let ptr = alloc(layout);

        // Set the allocated variable - dereference the pointer and set to 42
        *ptr = 42;
        assert_eq!(42, *ptr);

        // Free the memory - this is not automatic
        dealloc(ptr, layout);
    }
}
```

You have pretty much everything you expect from C: pointer arithmetic, `null` pointers, forgetting to call `dealloc` and leaking memory. At this level, it's quite ugly.