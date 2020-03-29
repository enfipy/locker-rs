# Locker

Utility `locker` - simple async/sync locker for rust concurrency.

## Async Example

```rust
use std::time::Duration;
use locker::AsyncLocker;
use tokio::time::delay_for;

let locker = AsyncLocker::new();
let mutex = locker.get_mutex(1).await;
let _guard = mutex.lock().await; // lock
let locker_clone = locker.clone();
tokio::spawn(async move {
    let mutex = locker.get_mutex(1).await;
    let _guard = mutex.lock().await; // wait
});
delay_for(Duration::from_millis(200)).await;
```

## Sync Example

```rust
use std::time::Duration;
use std::thread;
use locker::SyncLocker;

let locker = SyncLocker::new();
let mutex = locker.get_mutex(1);
let _guard = mutex.lock().unwrap(); // lock
let locker_clone = locker.clone();
thread::spawn(move || {
    let mutex = locker.get_mutex(1);
    let _guard = mutex.lock().unwrap(); // wait
});
thread::sleep(Duration::from_millis(200));
```

## Run test

To run tests:

```sh
cargo test -- --nocapture
```
