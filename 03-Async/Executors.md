# Different Executors

There's a lot of different executors available, so you can choose the one that fits what you need.

| **Executor** | **Description** |
| --- | --- |
| [Futures](https://docs.rs/futures/latest/futures/) | Core abstractions, proof-of-concept executor |
| [Executor](https://github.com/richardanaya/executor) | A minimal executor in 100 lines of code. Works with everything. Very few features. Can run on embedded, web-assembly, without the standard library. A great starting point if you need to implement your own for an embedded or WASM project. |
| [Async-Std](https://docs.rs/async-std/latest/async_std/) | A standard library for async Rust. As Async abstractions are finished, they are implemented in this library. Also includes a decently performing executor. Seeks maximum compatibility. |
| [Tokio](https://docs.rs/tokio/latest/tokio/) | Performance-oriented async executor and library. Continually updated, forms the core of Axum, Tungstenite (for web sockets) and other libraries. Includes tracing and telemetry support. The typical choice for enterprise applications. |

For the rest of this class, we'll use Tokio.