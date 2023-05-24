# Thread Priority

> The code for this is in the `thread_priorities` directory in `code/02_threads`.

This is another controversial topic! Most of the time, leave thread priority alone. The OS scheduler is pretty good at figuring out what to do. Sometimes, though, you need to control thread priority. A thread that always has a lot of work to do can benefit from being given a high priority. A thread that does a lot of waiting can benefit from being given a low priority.

Conversely, a thread that doesn't do much---but has a high priority---will waste lots of CPU time checking to see if it still idle!

## Pitfalls

* You can wind up with [priority inversion](https://en.wikipedia.org/wiki/Priority_inversion) by mistake. If a high priority task in some way depends on a low-priority task, despite being high-priority---the thread is effectively bounded by the lower-priority task.
* You can generate "starvation"---a high priority thread that activates regularly with nothing to do. This wastes CPU time.
* If you set everything to high priority, everything is effectively normal priority!

## Setting Thread Priority

Thread priority is not included in the Rust standard library. It's platform-specific, right down to the priority names! Add a crate to help you:

```bash
cargo add thread_priority
```

Here's an example:

```rust
use std::{sync::atomic::AtomicU32, time::Duration};
use thread_priority::*;

static LOW_COUNT: AtomicU32 = AtomicU32::new(0);
static MEDIUM_COUNT: AtomicU32 = AtomicU32::new(0);
static HIGH_COUNT: AtomicU32 = AtomicU32::new(0);

fn low_prio() {
    set_current_thread_priority(ThreadPriority::Min).unwrap();
    loop {
        LOW_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        std::thread::yield_now();
    }
}

fn regular_prio() {
    loop {
        MEDIUM_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        std::thread::yield_now();
    }
}

fn main() {
    std::thread::spawn(low_prio);
    std::thread::spawn(regular_prio);

    std::thread::sleep(Duration::from_secs(10));

    println!("Low    : {:>10}", LOW_COUNT.load(std::sync::atomic::Ordering::Relaxed));
    println!("Medium : {:>10}", MEDIUM_COUNT.load(std::sync::atomic::Ordering::Relaxed));
}
```

On my system this prints:

```
Low    :   99406604
Medium :   99572604
```

The differences are very small. They become a little more pronounced when you do a lot of work in your thread. It's not going to make a *massive* difference, modern OS schedulers do a LOT of work to maintain fairness.

## Combining CPU Priority With Affinity

In my experience, this is most useful when combined with affinity. A high priority thread on a core (not core 0!) is likely to keep that core mostly to itself.
