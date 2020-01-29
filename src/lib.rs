mod entry;

use crate::entry::*;
use std::borrow::Borrow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, SystemTime};

/// Represents a local in-memory cache.
pub struct MemoryCache<A, B> {
    table: HashMap<A, CacheEntry<B>>,
    full_scan_frequency: Duration,
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
    pub fn new(full_scan_frequency: Duration) -> MemoryCache<A, B> {
        MemoryCache::<A, B> {
            table: HashMap::<A, CacheEntry<B>>::new(),
            full_scan_frequency,
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

        self.table
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
    /// let scan_frequency = Duration::from_secs(60);
    ///
    /// let mut cache = MemoryCache::new(scan_frequency);
    ///
    /// let key: &'static str = "key";
    /// let value: &'static str = "Hello, World!";
    ///
    /// cache.set(key, value, None);
    ///
    /// assert_eq!(cache.get_last_scan_time(), None);
    /// ```
    pub fn get_last_scan_time(&self) -> Option<&SystemTime> {
        self.last_scan_time.as_ref()
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
    /// let mut cache = MemoryCache::new(scan_frequency);
    ///
    /// let key: &'static str = "key";
    /// let value: &'static str = "Hello, World!";
    ///
    /// cache.set(key, value, None);
    ///
    /// assert_eq!(cache.get_full_scan_frequency(), &scan_frequency);
    /// ```
    pub fn get_full_scan_frequency(&self) -> &Duration {
        self.full_scan_frequency.borrow()
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
            .filter(|cache_entry| !cache_entry.is_expired(now))
            .map(|cache_entry| cache_entry.value.borrow())
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
        let now = SystemTime::now();

        self.try_full_scan_expired_items(now);

        match self.table.entry(key) {
            Entry::Occupied(mut entry) => {
                if entry.get().is_expired(now) {
                    let cache_entry = CacheEntry::new(factory(), duration);
                    entry.insert(cache_entry);
                }

                entry.into_mut().value.borrow()
            }
            Entry::Vacant(entry) => {
                let cache_entry = CacheEntry::new(factory(), duration);
                entry.insert(cache_entry).value.borrow()
            }
        }
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
        let now = SystemTime::now();

        self.try_full_scan_expired_items(now);

        let cache_entry = CacheEntry::new(value, duration);
        self.table.insert(key, cache_entry);
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
        let now = SystemTime::now();

        self.try_full_scan_expired_items(now);

        self.table
            .remove(key)
            .filter(|cache_entry| !cache_entry.is_expired(now))
            .map(|cache_entry| cache_entry.value)
    }

    fn try_full_scan_expired_items(&mut self, current_time: SystemTime) {
        let since = match self.last_scan_time {
            Some(last_scan_time) => current_time.duration_since(last_scan_time).unwrap(),
            None => current_time.duration_since(self.created_time).unwrap(),
        };

        if since >= self.full_scan_frequency {
            self.table
                .retain(|_, cache_entry| !cache_entry.is_expired(current_time));

            self.last_scan_time = Some(current_time);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_key_with_expired_entry() {
        // Arrange
        let scan_frequency = Duration::from_secs(60);
        let mut cache = MemoryCache::new(scan_frequency);
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
        let scan_frequency = Duration::from_secs(60);
        let mut cache = MemoryCache::new(scan_frequency);
        let key: &'static str = "key";

        // Act
        cache.set(key, 1, Some(Duration::default()));
        let value = cache.get(&key);

        // Assert
        assert_eq!(value, None);
    }

    #[test]
    fn get_or_set_with_expired_entry() {
        // Arrange
        let scan_frequency = Duration::from_secs(60);
        let mut cache = MemoryCache::new(scan_frequency);
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
        let scan_frequency = Duration::from_secs(60);
        let mut cache = MemoryCache::new(scan_frequency);
        let key: &'static str = "key";

        // Act
        cache.set(key, 1, Some(Duration::default()));
        let value = cache.remove(&key);

        // Assert
        assert_eq!(value, None);
    }
}
