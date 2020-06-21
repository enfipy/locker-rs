use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

#[derive(Clone)]
pub struct AsyncLocker<K, V = ()> {
    default_mutex_func: Arc<dyn Fn() -> V + Send + Sync + 'static>,
    mutexes: Arc<RwLock<HashMap<K, Arc<Mutex<V>>>>>,
}

impl<K: Eq + Hash, V> AsyncLocker<K, V> {
    pub fn new(default_mutex_func: impl Fn() -> V + Send + Sync + 'static) -> Self {
        AsyncLocker {
            default_mutex_func: Arc::new(default_mutex_func),
            mutexes: Arc::new(RwLock::new(HashMap::<K, Arc<Mutex<V>>>::new())),
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
    /// let default_mutex_value = "value";
    /// let locker = AsyncLocker::<i32, &str>::new(move || default_mutex_value);
    /// let mutex = locker.get_mutex(1).await;
    /// let _guard = mutex.lock().await; // lock
    /// let locker_clone = locker.clone();
    /// tokio::spawn(async move {
    ///     let mutex = locker.get_mutex(1).await;
    ///     let value = mutex.lock().await; // wait
    ///     assert_eq!(default_mutex_value, *value);
    /// });
    /// delay_for(Duration::from_millis(200)).await;
    /// ```
    pub async fn get_mutex(&self, key: K) -> Arc<Mutex<V>> {
        {
            let mutexes = self.mutexes.read().await;
            let mutex_opt = mutexes.get(&key);
            if let Some(mutex) = mutex_opt {
                return mutex.clone();
            };
        }
        let mut mutexes = self.mutexes.write().await;
        let new_mutex = Arc::new(Mutex::new((self.default_mutex_func)()));
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
        let default_mutex_value = "value";
        let locker = AsyncLocker::<i32, &str>::new(move || default_mutex_value);
        let locker_clone = locker.clone();

        let handle = tokio::spawn(async move {
            let mutex = locker_clone.get_mutex(1).await;
            loop {
                println!("task mutex try to lock");
                match mutex.try_lock() {
                    Ok(value) => {
                        println!("task mutex locked");
                        assert_eq!(default_mutex_value, *value);
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
