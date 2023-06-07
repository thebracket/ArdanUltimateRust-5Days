# The `Unsafe` Keyword

Rust has a terrifying sounding keyword: `unsafe`. It's not as scary as it sounds: it *is* a way to tell the compiler "I know what I'm doing, and I'm taking responsibility for it! It also tells your code reviewers that they should take a really close look at the unsafe code.

## Rust is Safe by Default

Bjarne Stroustrup, the creator of C++, recently commented that C++ has most of the same safety mechanisms as Rust, but they are all opt-in. Rust is safe by default, and you have to opt-out of safety.

For example, the safest way to access a vector entry is to use get:

```rust
if let Ok(value) = my_vector.get(2) {
    // Do something
}
```

If there isn't a second element to the vector, nothing bad happens - you receive a `None` value and your code won't try and access it.

The `[]` method for accessing a vector is safe but panics:

```rust
fn main() {
    let my_vec = Vec::<u32>::new();
    println!("{}", my_vec[2]);
}
```

This will panic with the message `index out of bounds: the len is 0 but the index is 2` (with or without optimizations). You should save handling panics for exceptional circumstances, and not rely on them for normal program flow. Safe - normal - Rust won't let you access a vector entry out of bounds---so you won't inadvertently corrupt your program and crash (or worse!).

Buf if you *really* need the performance boost that comes from not checking bounds, you can use the `unsafe` keyword:

```rust
fn main() {
    let my_vec = Vec::<u32>::new();
    unsafe {
        println!("{}", my_vec.get_unchecked(2));
    }
}
```

The program crashes out with a `segmentation fault` (segfault) error. This can be really bad (an awful lot of C and C++ vulnerabilities start with an innocent looking segfault!)---but if you are *absolutely sure* that your bounds are correct, you *can* use `unsafe` to get a performance boost.

> C++ is the other way around. `[]` is the default and `at()` is the safe version. It's sadly rare to see `at()` used in C++ code.

## Unsafe Promises

There are two ways to utilize `unsafe`. You can mark a whole function as unsafe:

```rust
unsafe fn my_unsafe_function() {
    // Do something unsafe
}
```

Or you can have an `unsafe` block like we did above.

If you are using `RustDoc`, and the associated `missing_docs` warning then the documentation system will insist that any `unsafe` code be documented:

```rust
/// # Safety
/// This function calls into C code, and must be called with care. It is not thread-safe.
unsafe fn my_unsafe_function() {
    ...
}
```

This is a good thing. It forces you to think about what you are doing, and document it for others.

## So Why Am I Teaching You About Unsafe?

1. If you read the source code of other programs, you'll see `unsafe` in use. Every program has *some* unsafe code, somewhere. Rust can't extend its safety checks outside of Rust---so once you are physically interacting with hardware, using the platform's `libc` library, or calling into C code, you are in unsafe territory.
2. If a colleague submits some code with `unsafe` blocks and you need to review it, it's a good idea to know what they are doing.
3. You may find yourself needing it. If you're working on an embedded system, a high-performance library or similar---you might need it. It's best to wrap it up in a safe interface, document it well, and keep others from having to be unsafe.

I go out of my way to avoid using `unsafe`. If there's a well-respected library that can do it for me, I'll use it. If I have to use `unsafe`, I document and test it very heavily---and try to offer users a safe path to use the code. Please don't be cavalier about it and use `unsafe` everywhere in the name of saving a few nanoseconds of performance. Benchmark, test, and optimize *when you need to*.

> Premature Optimization is the Root of Most Security Vulnerabilities!
