use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Named `Mutex` handler
///
/// # Examples
///
/// ```
/// use std::sync::Arc;
///
/// let locker = Arc::new(Locker::new());
/// let locker_clone = locker.clone();
/// let mutex = locker.get_mutex("name");
/// // locks
/// let _ = mutex.lock.unwrap();
/// std::thread::spawn(move || {
///     let mutex = locker.get_mutex("name");
///     let _ = mutex.lock.unwrap(); // wait
/// });
/// ```
pub struct Locker {
    locks: Mutex<HashMap<String, Arc<Mutex<()>>>>,
}

impl Locker {
    pub fn new() -> Self {
        Locker {
            locks: Mutex::new(HashMap::new()),
        }
    }

    /// Gets reference to existig named `Mutex` or inserts  new one to `Locker` state.
    ///
    /// Blocking the current thread until it is able to do so.
    ///
    /// Then that `Mutex` can be used for locking thread.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    ///
    /// let locker = Arc::new(Locker::new());
    /// let locker_clone = locker.clone();
    /// let mutex = locker.get_mutex("name");
    /// // locks
    /// let _ = mutex.lock.unwrap();
    /// std::thread::spawn(move || {
    ///     let mutex = locker.get_mutex("name");
    ///     let _ = mutex.lock.unwrap(); // wait
    /// });
    /// ```
    pub fn get_mutex<N>(&self, name: N) -> Arc<Mutex<()>>
    where
        N: Into<String>,
    {
        let mut locks = self.locks.lock().unwrap();
        locks.entry(name.into()).or_insert(Arc::new(Mutex::new(()))).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::Locker;
    use std::sync::Arc;

    #[test]
    fn test_locker() {
        let handle = {
            let locker = Arc::new(Locker::new());
            let locker_clone = locker.clone();
            let handle = std::thread::spawn(move || {
                let mutex = locker_clone.get_mutex("name");
                loop {
                    println!("thread mutex try to lock");
                    match mutex.try_lock() {
                        Ok(_) => {
                            println!("thread mutex locked");
                            std::thread::sleep(std::time::Duration::from_secs(2));
                            println!("thread mutex unlocked");
                            break;
                        }
                        Err(_) => {
                            std::thread::sleep(std::time::Duration::from_millis(400));
                            println!("thread mutex wait unlock");
                            continue;
                        }
                    }
                }
            });
            let mutex = locker.get_mutex("name");
            loop {
                println!("main mutex try to lock");
                match mutex.try_lock() {
                    Ok(_) => {
                        println!("main mutex locked");
                        std::thread::sleep(std::time::Duration::from_secs(2));
                        println!("main mutex unlocked");
                        break;
                    }
                    Err(_) => {
                        std::thread::sleep(std::time::Duration::from_millis(400));
                        println!("main mutex wait for unlock");
                        continue;
                    }
                }
            }
            handle
        };
        handle.join().unwrap();
        assert!(true);
    }
}
