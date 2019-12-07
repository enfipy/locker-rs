use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
// use std::mem::drop;

pub type Lock = Arc<RwLock<()>>;

pub struct Locker {
    locks: Mutex<HashMap<String, Lock>>,
}

impl Locker {
    pub fn new() -> Self {
        Locker {
            locks: Mutex::new(HashMap::new()),
        }
    }

    pub fn acquire<S: Into<String>>(&mut self, key: S) -> Lock {
        let lock = Arc::new(RwLock::new(()));
        let mut locks = self.locks.lock().expect("FATAL: Failed to lock mutex of locker");
        locks.entry(key.into()).or_insert(lock).clone()
    }

    pub fn read<S: Into<String>>(&mut self, key: S) -> &() {
        let lock = self.acquire(key);
        // Todo: Return lock ref to unlock on end of reference life
        if let Ok(val) = lock.read() {
            return &();
        }
        return &();
    }
}

#[cfg(test)]
mod tests {
    use super::Locker;

    #[test]
    fn test_acquire() {
        let mut locker = Locker::new();
        let _lock = locker.acquire("example");
        // lock.read().unwrap();
        assert_eq!(0, 0);
    }

    #[test]
    fn test_read_lock() {
        let mut locker = Locker::new();
        let _lock = locker.read("example");
    }
}
