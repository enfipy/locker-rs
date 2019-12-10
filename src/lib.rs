use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Named `Mutex` handler
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
    /// use locker::Locker;
    ///
    /// let locker = Arc::new(Locker::new());
    /// let locker_clone = locker.clone();
    /// let name = "name";
    /// let mutex = locker.get_mutex(name); // locks
    /// let _ = mutex.lock().unwrap();
    /// std::thread::spawn(move || {
    ///     let mutex = locker.get_mutex(name);
    ///     let _ = mutex.lock().unwrap(); // wait
    /// });
    /// // unlocks first lock
    /// ```
    pub fn get_mutex<N>(&self, name: N) -> Arc<Mutex<()>>
    where
        N: Into<String>,
    {
        let mut locks = self.locks.lock().unwrap();
        let mutex = Arc::new(Mutex::new(()));
        locks.entry(name.into()).or_insert(mutex).clone()
    }

    pub fn lock_mutex<N, F, T, E>(&self, name: N, code: F) -> Result<T, E>
    where
        N: Into<String>,
        F: FnOnce() -> Result<T, E>,
        E: std::error::Error,
    {
        let mutex = self.get_mutex(name);
        let _ = mutex.lock().unwrap();
        code()
    }
}

#[cfg(test)]
mod tests {
    use super::Locker;
    use std::sync::Arc;

    #[test]
    fn test_locker() {
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
                        println!("thread mutex wait unlock");
                        std::thread::sleep(std::time::Duration::from_millis(400));
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
                    println!("main mutex wait for unlock");
                    std::thread::sleep(std::time::Duration::from_millis(400));
                    continue;
                }
            }
        }
        handle.join().unwrap()
    }

    #[test]
    fn test_lock_mutex() -> Result<(), std::io::Error> {
        let value = String::from("value");
        let locker = Arc::new(Locker::new());
        locker.lock_mutex("name", || {
            println!("thread mutex locked");
            std::thread::sleep(std::time::Duration::from_secs(2));
            println!("thread mutex unlocked");
            println!("{}", value);
            Ok(())
        })
    }
}
