# Locker

Utility `locker` - simple named mutex/locker for rust-lang concurrency with no dependencies.

## Example

Basic usage of `Locker`:

```rust
use std::sync::Arc;
use locker::Locker;

let locker = Arc::new(Locker::new());
let locker_clone = locker.clone();
let name = "name";
let first = locker.get_mutex(name); // locks
let _ = first.lock().unwrap();
std::thread::spawn(move || {
    let second = locker.get_mutex(name);
    let _ = second.lock().unwrap(); // wait
    // unlocks second mutex
});
// unlocks first mutex
```

## Run test

To run tests:

```sh
cargo test -- --nocapture
```
