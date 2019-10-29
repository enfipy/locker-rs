use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

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

    pub fn acquire<S>(&mut self, key: S) -> Lock
    where
        S: Into<String>,
    {
        let lock = Arc::new(RwLock::new(()));
        let mut locks = self.locks.lock().expect("FATAL: Failed to lock mutex of locker");
        locks.entry(key.into()).or_insert(lock).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::Locker;

    #[test]
    fn acquire_locker() {
        let mut locker = Locker::new();
        let _ = locker.acquire("example");
    }
}
