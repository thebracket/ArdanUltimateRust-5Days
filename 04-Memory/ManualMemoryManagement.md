# Why Haven't We Manually Managed Any Memory Yet?

When you talk to people about Rust, you often hear "it's a great, fast language - but so hard to learn, and so much memory management, fighting the borrow checker, etc.".

So far, we've barely done any memory management at all. You've *seen* a few details, but haven't really had to do any yourself. You've run into:

* `'static` lifetimes, attached to strings stored permanently in memory (in Axum).
* `Box`, used to wrap a dynamic `Result` type (which we promptly replaced with `anyhow`).
* `impl (trait)`, specifying that you are returning a type that implements a trait (e.g. `impl Future`) rather than a concrete type.
* `unsafe` in an example of how *not* to share data.

Other than that, you haven't had to add lifetime annotations, use `unsafe` code, allocate or de-allocate memory, or even free up resources (file handles, channels, etc.) when you are done with them.

*Why is that?*

"Idiomatic" Rust is very powerful and can go as low-level as C (you can even inline assembly!)---typically with safety guarantees on top. But it's also very high-level, and you can write code that is almost as high-level as Python or Ruby. "Normal" programming---building a web server, a command-line tool, a GUI app, a game, etc.---doesn't require you to do any memory management at all. The underlying systems libraries are taking care of it.

