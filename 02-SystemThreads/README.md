# Fearless System Thread Concurrency

> Please have Rust installed (via RustUp) and a working IDE/editor setup.

## First Hour

* [Introduction](./Intro.md)
* [System Threads: Overview](./SystemThreadOverview.md)
* [Create Your First Thread](./FirstThread.md)
* [Spawning Threads with Parameters and Closures](./ThreadClosures.md)
* [Returning Data from Threads](./ReturnFromThreads.md)
* [Dividing Workloads](./DividingWorkloads.md)
* [The ThreadBuilder Pattern](./ThreadBuilder.md)
* Let's take a break

## Second Hour

* [Scoped Threads for Easy Local Data Sharing](./ScopedThreads.md)
* [Sharing Data with Atomics](./Atomics.md)
* [Sharing Data with Mutexes](./Mutexes.md)
* [Read/Write Locks](./ReadWriteLocks.md)
* [Deadlocks, Panics and Poisoning](./Deadlocks.md)
* [Sharing Data with Lock-Free Structures](./LockFree.md)
* [Parking Threads](./ParkingThreads.md)
* Let's take a break

## Third Hour

* [Sending Data Between Threads with Channels](./Channels.md)
* [Channels and Ownership](./ChannelOwnership.md)
* [Sending Functions to Worker Threads](./SendingFunctions.md)
* [Let's build a work queue with a thread pool](./WorkQueue.md)
* [Thread CPU/Core Affinity](./ThreadAffinity.md)
* [Thread Priority](./ThreadPriority.md)
* Let's take a break

## Fourth Hour

* [Making it Easy with Rayon](./Rayon.md)
* [Scopes and Pooled Threads with Rayon](./RayonScopes.md)
* Wrap-Up & QA
* Next week... green threads and Tokio