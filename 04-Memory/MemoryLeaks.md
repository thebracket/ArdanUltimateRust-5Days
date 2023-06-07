# Memory Leaks

Surprise! Memory leaks are "safe"---in that Rust doesn't *guarantee* that it will prevent them. You saw one memory leak when we made a cycle of reference-counted objects. Rust can't prevent this, nor can it be reasonably detected without running the program---so it doesn't violate the safety guarantees of Rust.

If you want to leak memory, `std::mem::forget` explicitly does that! It's a function that takes ownership of a value and then does nothing with it. The value is never dropped, and the memory is never freed. This is useful for interfacing with C code that expects to take ownership of a value.

Despite this, as you've seen---it takes a bit of work to accidentally leak memory in Rust.