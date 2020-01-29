use std::time::{Duration, SystemTime};

/// Represents a set of eviction and expiration details for a specific cache entry.
pub(crate) struct CacheEntry<B> {
    /// Entry value.
    pub(crate) value: B,

    /// Expiration time.
    ///
    /// - [`None`] if the value must be kept forever.
    expiration: Option<SystemTime>,
}

impl<B> CacheEntry<B> {
    pub(crate) fn new(value: B, duration: Option<Duration>) -> Self {
        CacheEntry {
            expiration: duration.map(|d| SystemTime::now() + d),
            value,
        }
    }

    /// Check if a entry is expired.
    pub(crate) fn is_expired(&self, current_time: SystemTime) -> bool {
        match self.expiration {
            Some(time) => current_time >= time,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_expired() {
        // Arrange
        let now = SystemTime::now();

        let entry_expired = CacheEntry::new(1, Some(Duration::from_secs(0)));
        let entry_not_expired = CacheEntry::new(1, Some(Duration::from_secs(1)));
        let entry_none_duration = CacheEntry::new(1, None);

        // Act and Assert
        assert!(entry_expired.is_expired(now));
        assert!(!entry_not_expired.is_expired(now));
        assert!(!entry_none_duration.is_expired(now));
    }
}
