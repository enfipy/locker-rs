# Locker

Utility `locker` - simple async mutex/locker for rust concurrency.

## Example

Basic usage of `Locker`:

```rust
use std::time::Duration;
use locker::Locker;
use tokio::time::delay_for;

let locker = Locker::new();
let mutex = locker.get_mutex(1).await;
let _guard = mutex.lock().await; // lock
let locker_clone = locker.clone();
tokio::spawn(async move {
    let mutex = locker.get_mutex(1).await;
    let _guard = mutex.lock().await; // wait
});
delay_for(Duration::from_millis(200)).await;
```

## Run test

To run tests:

```sh
cargo test -- --nocapture
```
