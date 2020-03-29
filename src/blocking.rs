use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex, RwLock};

#[derive(Clone)]
pub struct SyncLocker<K> {
    mutexes: Arc<RwLock<HashMap<K, Arc<Mutex<()>>>>>,
}

impl<K: Eq + Hash> SyncLocker<K> {
    pub fn new() -> Self {
        SyncLocker {
            mutexes: Arc::new(RwLock::new(HashMap::<K, Arc<Mutex<()>>>::new())),
        }
    }

    /// Return reference to existig `Mutex` or insert new one.
    ///
    /// Locks the current task until it is able to return `Mutex`.
    ///
    /// # Examples
    /// ```ignore
    /// use std::time::Duration;
    /// use std::thread;
    /// use locker::SyncLocker;
    ///
    /// let locker = SyncLocker::new();
    /// let mutex = locker.get_mutex(1);
    /// let _guard = mutex.lock().unwrap(); // lock
    /// let locker_clone = locker.clone();
    /// thread::spawn(move || {
    ///     let mutex = locker.get_mutex(1);
    ///     let _guard = mutex.lock().unwrap(); // wait
    /// });
    /// thread::sleep(Duration::from_millis(200));
    /// ```
    pub fn get_mutex(&self, key: K) -> Arc<Mutex<()>> {
        {
            let mutexes = self.mutexes.read().unwrap();
            let mutex_opt = mutexes.get(&key);
            if let Some(mutex) = mutex_opt {
                return mutex.clone();
            };
        }
        let mut mutexes = self.mutexes.write().unwrap();
        let new_mutex = Arc::new(Mutex::new(()));
        mutexes.entry(key).or_insert(new_mutex).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::SyncLocker;
    use std::thread;
    use std::time::Duration;

    fn test_sync_locker() {
        let locker = SyncLocker::new();
        let locker_clone = locker.clone();

        let handle = thread::spawn(move || {
            let mutex = locker_clone.get_mutex(1);
            loop {
                println!("thread mutex try to lock");
                match mutex.try_lock() {
                    Ok(_) => {
                        println!("thread mutex locked");
                        thread::sleep(Duration::from_millis(100));
                        println!("thread mutex unlocked");
                        break;
                    }
                    Err(_) => {
                        println!("thread mutex wait unlock");
                        thread::sleep(Duration::from_millis(100));
                        continue;
                    }
                }
            }
        });

        let mutex = locker.get_mutex(1);
        loop {
            println!("main mutex try to lock");
            match mutex.try_lock() {
                Ok(_) => {
                    println!("main mutex locked");
                    thread::sleep(Duration::from_millis(100));
                    println!("main mutex unlocked");
                    break;
                }
                Err(_) => {
                    println!("main mutex wait for unlock");
                    thread::sleep(Duration::from_millis(100));
                    continue;
                }
            }
        }

        handle.join().unwrap();
    }
}
