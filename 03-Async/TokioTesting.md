# Unit Testing with Tokio

You've seen before how easy Rust makes it to include unit tests in your project. Tokio makes it just as easy to test asynchronous code.

As a reminder, here's a regular---synchronous---unit test:

```rust
fn main() {
}

#[cfg(test)]
mod test {
    #[test]
    fn simple_test() {
        assert_eq!(2 + 2, 4);
    }
}
```

Run the test with `cargo test`, and you prove that your computer can add 2 and 2:

```
running 1 test
test test::simple_test ... ok
```

## The Problem with Async Tests

The problem with using the regular `#[test]` syntax is that you can't use `async` functions from a synchronous context. It won't compile:

```rust
fn main() {
}

async fn double(n: i32) -> i32 {
    n * 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn will_not_work() {
        // This will not work because we are not in an async context
        let result = double(2);
        assert_eq!(result, 4);
    }
}
```

## Option 1: Build a context in each test

The long-form solution is to build an async context in each test. For example:

```rust
#[test]
fn the_hard_way() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    assert_eq!(rt.block_on(double(2)), 4);
}
```

When you start doing this in every test, you wind up with a huge set of unit tests---and a lot of boilerplate. Fortunately, Tokio provides a better way.

## Option 2: Use `tokio-test`

Tokio provides an alternative test macro---like like the `tokio::main` macro---for your tests. It adds the boilerplate to build an async context for you:

```rust
#[tokio::test]
async fn the_easy_way() {
    assert_eq!(double(2).await, 4);
}
```

The `tokio::test` macro creates a full multi-threaded runtime for you. It's a great way to get started with Tokio testing. You can use it as a full async context---awaiting, joining, spawning.

## Option 3: Single-threaded Tokio Test

If you are executing in a single-threaded environment, you also want to test single-threaded. Testing single-threaded can also be a good way to catch those times you accidentally blocked.

To test single-threaded, use the `tokio::test` macro with the `single_thread` feature:

```rust
#[tokio::test(flavor = "current_thread")]
async fn single_thread_tokio() {
    assert_eq!(double(2).await, 4);
}
```

## Taming Multi-Threaded Tokio

Rust unit tests already run in a threaded context (multiple tests execute at once)---creating a thread pool encompassing every CPU your system has for a test is probably overkill. You can also decorate the `tokio::test` macro with the `multi_thread` feature to create a multi-threaded Tokio runtime with a limited number of threads:

```rust
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn tamed_multi_thread() {
    assert_eq!(double(2).await, 4);
}
```

