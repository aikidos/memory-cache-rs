mod entry;
pub mod macros;

use crate::entry::*;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, SystemTime};

/// Represents a local in-memory cache.
pub struct MemoryCache<A, B> {
    cache_table: HashMap<A, CacheEntry<B>>,
    full_scan_frequency: Option<Duration>,
    created_time: SystemTime,
    last_scan_time: Option<SystemTime>,
}

impl<A: Hash + Eq, B> MemoryCache<A, B> {
    /// Creates an empty `MemoryCache`.
    ///
    /// # Example
    /// ```
    /// use memory_cache::MemoryCache;
    /// use std::time::Duration;
    ///
    /// let mut cache = MemoryCache::new();
    ///
    /// let key: &'static str = "key";
    /// let value: &'static str = "Hello, World!";
    ///
    /// cache.insert(key, value, None);
    ///
    /// assert_eq!(cache.get(&key), Some(&value));
    /// ```
    pub fn new() -> Self {
        Self {
            cache_table: HashMap::new(),
            full_scan_frequency: None,
            created_time: SystemTime::now(),
            last_scan_time: None,
        }
    }

    /// Creates an empty `MemoryCache` with periodic full scan to identify expired keys.
    ///
    /// # Example
    /// ```
    /// use memory_cache::MemoryCache;
    /// use std::time::Duration;
    ///
    /// let scan_frequency = Duration::from_secs(60);
    ///
    /// let mut cache = MemoryCache::with_full_scan(scan_frequency);
    ///
    /// let key: &'static str = "key";
    /// let value: &'static str = "Hello, World!";
    ///
    /// cache.insert(key, value, None);
    ///
    /// assert_eq!(cache.get(&key), Some(&value));
    /// ```
    pub fn with_full_scan(full_scan_frequency: Duration) -> Self {
        Self {
            cache_table: HashMap::new(),
            full_scan_frequency: Some(full_scan_frequency),
            created_time: SystemTime::now(),
            last_scan_time: None,
        }
    }

    /// Determines whether the `MemoryCache<A, B>` contains the specified key.
    ///
    /// # Example
    /// ```
    /// use memory_cache::MemoryCache;
    /// use std::time::Duration;
    ///
    /// let mut cache = MemoryCache::new();
    ///
    /// let key: &'static str = "key";
    /// let value: &'static str = "Hello, World!";
    ///
    /// cache.insert(key, value, None);
    ///
    /// assert!(cache.contains_key(&key));
    /// ```
    pub fn contains_key(&self, key: &A) -> bool {
        let now = SystemTime::now();

        self.cache_table
            .get(key)
            .filter(|cache_entry| !cache_entry.is_expired(now))
            .is_some()
    }

    /// Gets the last scan time.
    ///
    /// - [`None`] If there were no scans.
    ///
    /// # Example
    /// ```
    /// use memory_cache::MemoryCache;
    /// use std::time::{Duration, SystemTime};
    ///
    /// let mut cache = MemoryCache::new();
    ///
    /// let key: &'static str = "key";
    /// let value: &'static str = "Hello, World!";
    ///
    /// cache.insert(key, value, None);
    ///
    /// assert_eq!(cache.get_last_scan_time(), None);
    /// ```
    pub fn get_last_scan_time(&self) -> Option<SystemTime> {
        self.last_scan_time
    }

    /// Gets the full scan frequency.
    ///
    /// # Example
    /// ```
    /// use memory_cache::MemoryCache;
    /// use std::time::{Duration, SystemTime};
    ///
    /// let scan_frequency = Duration::from_secs(60);
    ///
    /// let mut cache = MemoryCache::with_full_scan(scan_frequency);
    ///
    /// let key: &'static str = "key";
    /// let value: &'static str = "Hello, World!";
    ///
    /// cache.insert(key, value, None);
    ///
    /// assert_eq!(cache.get_full_scan_frequency(), Some(scan_frequency));
    /// ```
    pub fn get_full_scan_frequency(&self) -> Option<Duration> {
        self.full_scan_frequency
    }

    /// Gets the value associated with the specified key.
    ///
    /// # Example
    /// ```
    /// use memory_cache::MemoryCache;
    /// use std::time::Duration;
    ///
    /// let mut cache = MemoryCache::new();
    ///
    /// let key: &'static str = "key";
    /// let value: &'static str = "Hello, World!";
    ///
    /// cache.insert(key, value, None);
    ///
    /// assert_eq!(cache.get(&key), Some(&value));
    /// ```
    pub fn get(&self, key: &A) -> Option<&B> {
        let now = SystemTime::now();

        self.cache_table
            .get(&key)
            .filter(|cache_entry| !cache_entry.is_expired(now))
            .map(|cache_entry| &cache_entry.value)
    }

    /// Gets the value associated with the specified key, or if the key can not be found,
    /// creates and insert value using the `factory` function.
    ///
    /// # Example
    /// ```
    /// use memory_cache::MemoryCache;
    /// use std::time::Duration;
    ///
    /// let mut cache = MemoryCache::new();
    ///
    /// let key: &'static str = "key";
    /// let value: &'static str = "Hello, World!";
    ///
    /// assert_eq!(cache.get_or_insert(key, move || value, None), &value);
    /// assert!(cache.contains_key(&key));
    /// ```
    pub fn get_or_insert<F>(&mut self, key: A, factory: F, lifetime: Option<Duration>) -> &B
    where
        F: Fn() -> B,
    {
        let now = SystemTime::now();

        self.try_full_scan_expired_items(now);

        match self.cache_table.entry(key) {
            Entry::Occupied(mut occupied) => {
                if occupied.get().is_expired(now) {
                    occupied.insert(CacheEntry::new(factory(), lifetime));
                }

                &occupied.into_mut().value
            }
            Entry::Vacant(vacant) => &vacant.insert(CacheEntry::new(factory(), lifetime)).value,
        }
    }

    /// Inserts a key-value pair into the cache.
    ///
    /// If the cache did not have this key present, `None` is returned.  
    /// If the cache did have this key present, the value is updated, and the old value is returned.
    ///
    /// # Example
    /// ```
    /// use memory_cache::MemoryCache;
    /// use std::time::Duration;
    ///
    /// let mut cache = MemoryCache::new();
    ///
    /// let key: &'static str = "key";
    /// let value: &'static str = "Hello, World!";
    ///
    /// cache.insert(key, value, None);
    ///
    /// assert_eq!(cache.get(&key), Some(&value));
    /// ```
    pub fn insert(&mut self, key: A, value: B, lifetime: Option<Duration>) -> Option<B> {
        let now = SystemTime::now();

        self.try_full_scan_expired_items(now);

        self.cache_table
            .insert(key, CacheEntry::new(value, lifetime))
            .filter(|cache_entry| !cache_entry.is_expired(now))
            .map(|cache_entry| cache_entry.value)
    }

    /// Removes a key from the cache, returning the value at the key if the key was previously in the cache.
    ///
    /// # Example
    /// ```
    /// use memory_cache::MemoryCache;
    /// use std::time::Duration;
    ///
    /// let mut cache = MemoryCache::new();
    ///
    /// let key: &'static str = "key";
    /// let value: &'static str = "Hello, World!";
    ///
    /// cache.insert(key, value, None);
    ///
    /// assert_eq!(cache.remove(&key), Some(value));
    /// assert!(!cache.contains_key(&key));
    /// ```
    pub fn remove(&mut self, key: &A) -> Option<B> {
        let now = SystemTime::now();

        self.try_full_scan_expired_items(now);

        self.cache_table
            .remove(key)
            .filter(|cache_entry| !cache_entry.is_expired(now))
            .map(|cache_entry| cache_entry.value)
    }

    fn try_full_scan_expired_items(&mut self, current_time: SystemTime) {
        if let Some(full_scan_frequency) = self.full_scan_frequency {
            let since = current_time
                .duration_since(self.last_scan_time.unwrap_or(self.created_time))
                .unwrap();

            if since >= full_scan_frequency {
                self.cache_table
                    .retain(|_, cache_entry| !cache_entry.is_expired(current_time));

                self.last_scan_time = Some(current_time);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_key_expired_entry() {
        // Arrange
        let mut cache = MemoryCache::new();
        let key: &'static str = "key";

        // Act
        cache.insert(key, 1, Some(Duration::default()));

        // Assert
        assert!(!cache.contains_key(&key));
    }

    #[test]
    fn get_expired_entry() {
        // Arrange
        let mut cache = MemoryCache::new();
        let key: &'static str = "key";

        // Act
        cache.insert(key, 1, Some(Duration::default()));

        // Assert
        assert_eq!(cache.get(&key), None);
    }

    #[test]
    fn insert_return_old_value() {
        // Arrange
        let mut cache = MemoryCache::new();
        let key: &'static str = "key";

        // Act
        let result_1 = cache.insert(key, 1, Some(Duration::default()));
        let result_2 = cache.insert(key, 2, None);
        let result_3 = cache.insert(key, 3, None);

        // Assert
        assert_eq!(result_1, None);
        assert_eq!(result_2, None);
        assert_eq!(result_3, Some(2));
    }

    #[test]
    fn get_or_insert_expired_entry() {
        // Arrange
        let mut cache = MemoryCache::new();
        let key: &'static str = "key";

        // Act
        cache.get_or_insert(key, || 1, Some(Duration::default()));
        let value = cache.get_or_insert(key, || 2, None);

        // Assert
        assert_eq!(value, &2);
    }

    #[test]
    fn remove_expired_entry() {
        // Arrange
        let mut cache = MemoryCache::new();
        let key: &'static str = "key";

        // Act
        cache.insert(key, 1, Some(Duration::default()));
        let value = cache.remove(&key);

        // Assert
        assert_eq!(value, None);
    }

    #[test]
    fn update_last_scan_time() {
        // Arrange
        let scan_frequency = Duration::default();
        let mut cache = MemoryCache::with_full_scan(scan_frequency);
        let key: &'static str = "key";

        // Act
        cache.insert(key, 1, None);
        let last_scan_time = cache.get_last_scan_time();

        // Assert
        assert!(last_scan_time.is_some())
    }
}
