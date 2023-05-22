# Fearless System Thread Concurrency

> Please have Rust installed (via RustUp) and a working IDE/editor setup.

## First Hour

* [Introduction](./Intro.md)
* [System Threads: Overview](./SystemThreadOverview.md)
* [Create Your First Thread](./FirstThread.md)
* [Spawning Threads with Parameters and Closures](./ThreadClosures.md)
* [Returning Data from Threads](./ReturnFromThreads.md)
* [Dividing Workloads](./DividingWorkloads.md)
* [Scoped Threads for Easy Local Data Sharing](./ScopedThreads.md)
* Let's take a break

## Second Hour

* [Sharing Data with Atomics](./Atomics.md)
* [Sharing Data with Mutexes](./Mutexes.md)
* [Read/Write Locks](./ReadWriteLocks.md)
* [Sharing Data with Lock-Free Structures](./LockFree.md)
* [Parking Threads](./ParkingThreads.md)
* Let's take a break

## Third Hour

* [Sending Data Between Threads with Channels](./Channels.md)
* [Channels and Ownership](./ChannelOwnership.md)
* [Sending Functions to Worker Threads](./SendingFunctions.md)
* [Let's build a work queue with a thread pool](./WorkQueue.md)
* Let's take a break

## Fourth Hour

* [Making it Easy with Rayon](./Rayon.md)
* Wrap-Up & QA
* Next week... green threads and Tokio