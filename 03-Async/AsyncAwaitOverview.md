# Async/Await Overview

> This is mostly theory, but we'll be writing some code in the next section.

Last week, we talked about [System Threads](../02-SystemThreads/README.md). System threads are managed by the operating system and are *preemptively* multi-tasked. What does that really mean?

* A thread can be interrupted at any time by the OS scheduler.
* The OS scheduler is relatively heavy-weight: it has to save the state of the current thread, load the state of the next thread, and then resume execution.
* You have limited control over when tasks are scheduled.

An *async* model is *cooperatively* multi-tasked, and may run in just one thread---or it may split tasks across many threads. What does that really mean?

* A task can only be interrupted when it yields control. (The executor process might still be interrupted by the OS scheduler.)
* Tasks are really light-weight: they just contain the execution stack (local variables and function calls), and any data indicating how to resume the task (for example---when a network operation completes).

## When to use Async or System Threads?

| **System Threads** | **Async** |
| --- | --- |
| Long-running tasks | Short-running tasks |
| CPU-bound tasks | I/O-bound tasks |
| Tasks that need to run in parallel | Tasks that need to run concurrently |
| Tasks that need minimal, predictable latency | Tasks that can take advantage of latency to do other things - and improve thoughput |

As Bill puts it: "Async takes advantage of latency". When you have a lot to do, and much of it involves waiting for something else to finish---a database operation, the network, a file system operation---then async is a great fit. It's also a great fit for short-running tasks that need to run concurrently.

**Don't** use Async when your task will consume lots of CPU and there may be a long pause between logical points to yield to other async tasks. You'll slow everything down.

**Don't** use System Threads to spawn one per network client and spend most of the time waiting for I/O. You'll run out of memory or thread allocations.

**Do** mix and match the two to really unlock the power of Rust's concurrency model.

## Rust and Async/Await

NodeJS, Go, C#, Python and others all implement an *opinionated* and *batteries included* approach to Async/Await. You `await` a "future" or "promise"---depending upon the language---and the language framework handles the rest.

C++ and Rust both took a more agnostic approach. They provide the building blocks, but leave it up to you to assemble them into a framework. This is a good thing, because it means you can build a framework that fits your needs, and not the needs of the language designers. It also allows for competition between frameworks, driving innovation and fitness for purpose.

The downside is that it's a bit more work to get started. But that's what this course is for!