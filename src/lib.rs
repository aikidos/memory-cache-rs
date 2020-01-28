mod entry;

use crate::entry::*;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, SystemTime};

/// Represents a local in-memory cache.
pub struct MemoryCache<A, B> {
    table: HashMap<A, CacheEntry<B>>,
    expiration_scan_frequency: Duration,
    last_scan_time: SystemTime,
}

impl<A: Hash + Eq, B> MemoryCache<A, B> {
    /// Creates an empty `MemoryCache`.
    ///
    /// # Example
    /// ```
    /// use memory_cache::MemoryCache;
    /// use std::time::Duration;
    ///
    /// let scan_frequency = Duration::from_secs(60);
    ///
    /// let mut cache = MemoryCache::new(scan_frequency);
    ///
    /// let key: &'static str = "key";
    /// let value: &'static str = "Hello, World!";
    ///
    /// cache.set(key, value, None);
    /// let cached_value = cache.get(&key);
    ///
    /// assert_eq!(cached_value, Some(&value));
    /// ```
    pub fn new(expiration_scan_frequency: Duration) -> MemoryCache<A, B> {
        MemoryCache::<A, B> {
            table: HashMap::<A, CacheEntry<B>>::new(),
            expiration_scan_frequency,
            last_scan_time: SystemTime::now(),
        }
    }

    /// Determines whether the `MemoryCache<A, B>` contains the specified key.
    ///
    /// # Example
    /// ```
    /// use memory_cache::MemoryCache;
    /// use std::time::Duration;
    ///
    /// let scan_frequency = Duration::from_secs(60);
    ///
    /// let mut cache = MemoryCache::new(scan_frequency);
    ///
    /// let key: &'static str = "key";
    ///
    /// cache.set(key, "Hello, World!", None);
    ///
    /// assert!(cache.has_key(&key));
    /// ```
    pub fn has_key(&self, key: &A) -> bool {
        let now = SystemTime::now();

        self.table.get(key).filter(|e| !e.is_expired(now)).is_some()
    }

    /// Gets the value associated with the specified key.
    ///
    /// # Example
    /// ```
    /// use memory_cache::MemoryCache;
    /// use std::time::Duration;
    ///
    /// let scan_frequency = Duration::from_secs(60);
    ///
    /// let mut cache = MemoryCache::new(scan_frequency);
    ///
    /// let key: &'static str = "key";
    /// let value: &'static str = "Hello, World!";
    ///
    /// cache.set(key, value, None);
    ///
    /// assert_eq!(cache.get(&key), Some(&value));
    /// ```
    pub fn get(&self, key: &A) -> Option<&B> {
        let now = SystemTime::now();

        self.table
            .get(&key)
            .filter(|e| !e.is_expired(now))
            .map(|entry| entry.value.borrow())
    }

    /// Gets the value associated with the specified key, or if the key can not be found,
    /// creates and insert value using the `factory` function.
    ///
    /// # Example
    /// ```
    /// use memory_cache::MemoryCache;
    /// use std::time::Duration;
    ///
    /// let scan_frequency = Duration::from_secs(60);
    ///
    /// let mut cache = MemoryCache::new(scan_frequency);
    ///
    /// let key: &'static str = "key";
    /// let value: &'static str = "Hello, World!";
    ///
    /// assert_eq!(cache.get_or_set(key, move || value, None), &value);
    /// assert!(cache.has_key(&key));
    /// ```
    pub fn get_or_set<F>(&mut self, key: A, factory: F, duration: Option<Duration>) -> &B
    where
        F: Fn() -> B,
    {
        self.remove_expired_items();

        self.table
            .entry(key)
            .or_insert_with(|| CacheEntry::new(factory(), duration))
            .value
            .borrow()
    }

    /// Inserts a cache entry into the cache by using a key and a value.
    ///
    /// # Example
    /// ```
    /// use memory_cache::MemoryCache;
    /// use std::time::Duration;
    ///
    /// let scan_frequency = Duration::from_secs(60);
    ///
    /// let mut cache = MemoryCache::new(scan_frequency);
    ///
    /// let key: &'static str = "key";
    ///
    /// cache.set(key, "Hello, World!", None);
    ///
    /// assert!(cache.has_key(&key));
    /// ```
    pub fn set(&mut self, key: A, value: B, duration: Option<Duration>) {
        self.remove_expired_items();

        let entry = CacheEntry::new(value, duration);

        self.table.insert(key, entry);
    }

    /// Removes a cache entry from the cache, returning the value at the key if the key
    /// was previously in the cache.
    ///
    /// # Example
    /// ```
    /// use memory_cache::MemoryCache;
    /// use std::time::Duration;
    ///
    /// let scan_frequency = Duration::from_secs(60);
    ///
    /// let mut cache = MemoryCache::new(scan_frequency);
    ///
    /// let key: &'static str = "key";
    /// let value: &'static str = "Hello, World!";
    ///
    /// cache.set(key, value, None);
    ///
    /// assert_eq!(cache.remove(&key), Some(value));
    /// assert!(!cache.has_key(&key));
    /// ```
    pub fn remove(&mut self, key: &A) -> Option<B> {
        self.remove_expired_items();

        self.table.remove(key).map(|e| e.value)
    }

    fn remove_expired_items(&mut self) {
        let now = SystemTime::now();

        if now.duration_since(self.last_scan_time).unwrap() >= self.expiration_scan_frequency {
            self.table.retain(|_, entry| !entry.is_expired(now));
            self.last_scan_time = now;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_key_with_expired_entry() {
        // Arrange
        let zero_duration = Duration::default();
        let mut cache = MemoryCache::new(zero_duration);
        let key: &'static str = "key";

        // Act
        cache.set(key, 1, Some(Duration::default()));
        let has_key = cache.has_key(&key);

        // Assert
        assert!(!has_key);
    }

    #[test]
    fn get_with_expired_entry() {
        // Arrange
        let zero_duration = Duration::default();
        let mut cache = MemoryCache::new(zero_duration);
        let key: &'static str = "key";

        // Act
        cache.set(key, 1, Some(Duration::default()));
        let value = cache.get(&key);

        // Assert
        assert_eq!(value, None);
    }

    #[test]
    fn get_or_create_with_expired_entry() {
        // Arrange
        let zero_duration = Duration::default();
        let mut cache = MemoryCache::new(zero_duration);
        let key: &'static str = "key";

        // Act
        cache.get_or_set(key, || 1, Some(Duration::default()));
        let value = cache.get_or_set(key, || 2, None);

        // Assert
        assert_eq!(value, &2);
    }

    #[test]
    fn remove_with_expired_entry() {
        // Arrange
        let zero_duration = Duration::default();
        let mut cache = MemoryCache::new(zero_duration);
        let key: &'static str = "key";

        // Act
        cache.set(key, 1, Some(Duration::default()));
        let value = cache.remove(&key);

        // Assert
        assert_eq!(value, None);
    }
}
