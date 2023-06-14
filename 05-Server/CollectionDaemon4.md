# Preventing Unbounded Growth

> The code for this is in `code/05_server/collector_v4`.

## Bounding the Send Queue

We still have a bit of cleaning to do on the collector. We are allowing unbounded growth, so a *long* disconnect from the server would cause memory to grow indefinitely.

We can add a very simple stanza to ensure that we never exceed our initially allocated `VecDeque` capacity:

```rust
if send_queue.len() > 120 {
    // Drop the first entry
    send_queue.pop_front();
}
send_queue.push_back(encoded);
```

The collector will no longer grow beyond the size of 120 command entries.

## Bounding the Channel

If things go wrong, the sender channel could become large. We're also allocating an unknown amount of data for the channel by allowing it to hold an infinite size.

Let's change the channel declaration to read:

```rust
let (tx, rx) = std::sync::mpsc::sync_channel::<CollectorCommandV1>(1);
```

To match, you have to change the data collector's parameters:

```rust
pub fn collect_data(tx: SyncSender<CollectorCommandV1>, collector_id: u128) {
```

Let it run, and inspect the process. Memory usage no longer changes while it runs, even though there's nothing to talk to.