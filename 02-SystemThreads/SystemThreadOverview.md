# System Thread Overview

> This is a quick bit of theory to get started.

## What is a System Thread

A system thread is an Operating System-level construct that allows you to run multiple tasks at the same time (or if you only have 1 CPU, fake it pretty well).

Threads live inside their parent process, and share memory with it. Each thread has its own stack, but they all share the same heap.

Threads are **preemptively* scheduled by the Operating System. This means:

* The Operating System decides when to switch between threads. You have limited control over actual scheduling (but can influence it with priorities, pinning and similar).
* The Operating System can interrupt a thread at any time, and switch to another thread. This means that you need to be careful about shared data, and make sure that you're not in the middle of a write when the OS switches to another thread. Rust protects you from yourself here.
* Threads are pretty lightweight, compared to processes---but there is overhead in creating one. On Linux, making a thread requires a call to a system call named `clone`, which sets up an ID for the scheduler, initializes the thread's stack. It can take 9,000 or more nanoseconds (Skylake) to create a thread (it's even worse on ARM; 20,000 nanoseconds on Ampere).
* You can't have huge numbers of threads. On Linux, `/proc/sys/kernel/threads-max` lists the maximum number of threads you can create. On my test Linux system, it's 63,704. On my Windows system, it's closer to 8,000.
* You don't want to have thousands of threads: the Operating System shares execution time between them, and execution can slow to a crawl when you have thousands of them.

So the takeaways:

1. Thread creation is relatively slow. Only make a thread for a task that once divided will justify the cost---or because you need to keep a task going in the background.
2. Thread creation is relatively expensive, with each thread getting its own stack. Don't make thousands of threads. Try to reuse threads.
3. Threads are a nice, easy way to divide workloads.
4. Threads are great when you want to perform calculations in the background and retain interactivity.

When you *do* want thousands of tasks, you often want to use an async/await model. We'll cover that next week.