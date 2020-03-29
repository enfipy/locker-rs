use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

#[derive(Clone)]
pub struct AsyncLocker<K> {
    mutexes: Arc<RwLock<HashMap<K, Arc<Mutex<()>>>>>,
}

impl<K: Eq + Hash> AsyncLocker<K> {
    pub fn new() -> Self {
        AsyncLocker {
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
    /// use locker::AsyncLocker;
    /// use tokio::time::delay_for;
    ///
    /// let locker = Locker::new();
    /// let mutex = locker.get_mutex(1).await;
    /// let _guard = mutex.lock().await; // lock
    /// let locker_clone = locker.clone();
    /// tokio::spawn(async move {
    ///     let mutex = locker.get_mutex(1).await;
    ///     let _guard = mutex.lock().await; // wait
    /// });
    /// delay_for(Duration::from_millis(200)).await;
    /// ```
    pub async fn get_mutex(&self, key: K) -> Arc<Mutex<()>> {
        {
            let mutexes = self.mutexes.read().await;
            let mutex_opt = mutexes.get(&key);
            if let Some(mutex) = mutex_opt {
                return mutex.clone();
            };
        }
        let mut mutexes = self.mutexes.write().await;
        let new_mutex = Arc::new(Mutex::new(()));
        mutexes.entry(key).or_insert(new_mutex).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::AsyncLocker;
    use std::time::Duration;
    use tokio::time::delay_for;

    #[tokio::test]
    async fn test_async_locker() {
        let locker = AsyncLocker::new();
        let locker_clone = locker.clone();

        let handle = tokio::spawn(async move {
            let mutex = locker_clone.get_mutex(1).await;
            loop {
                println!("task mutex try to lock");
                match mutex.try_lock() {
                    Ok(_) => {
                        println!("task mutex locked");
                        delay_for(Duration::from_millis(100)).await;
                        println!("task mutex unlocked");
                        break;
                    }
                    Err(_) => {
                        println!("task mutex wait unlock");
                        delay_for(Duration::from_millis(100)).await;
                        continue;
                    }
                }
            }
        });

        let mutex = locker.get_mutex(1).await;
        loop {
            println!("main mutex try to lock");
            match mutex.try_lock() {
                Ok(_) => {
                    println!("main mutex locked");
                    delay_for(Duration::from_millis(100)).await;
                    println!("main mutex unlocked");
                    break;
                }
                Err(_) => {
                    println!("main mutex wait for unlock");
                    delay_for(Duration::from_millis(100)).await;
                    continue;
                }
            }
        }

        handle.await.unwrap();
    }
}
