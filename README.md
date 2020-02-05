[memory-cache-rs](https://docs.rs/memory-cache-rs)
===

Simple local in-memory cache for [Rust](https://www.rust-lang.org/).

1. [Example](#example)
1. [Memoization](#memoization)
1. [Breaking Changes](#breaking-changes)
1. [Licence](#licence)

Example
---

```rust
use std::time::Duration;
use memory_cache::MemoryCache;

let mut cache = MemoryCache::new();

let key: &'static str = "key";
let value: &'static str = "Hello, World!";

// `None` - if the value must be kept forever.
let lifetime = Some(Duration::from_secs(30));

cache.insert(key, value, lifetime);

assert_eq!(cache.get(&key), Some(&value));
```

[Memoization](https://en.wikipedia.org/wiki/Memoization)
---
```rust
use once_cell::sync::Lazy;
use std::sync::Mutex;
use memory_cache::{MemoryCache, cached};

cached! {
    fn factorial(x: u128) -> u128 = {
        if x <= 1 {
            1
        } else {
            x * factorial(x - 1)
        }
    }
}

assert_eq!(factorial(21), 51090942171709440000);
```

Breaking Changes:
===

0.2.0:
---

**Constructors:**

```diff
- MemoryCache::new(full_scan_frequency: Duration) -> Self
+ MemoryCache::new() -> Self
+ MemoryCache::with_full_scan(full_scan_frequency: Duration) -> Self
```

**Renamed Methods:**

To look like a `HashMap`.

```diff
MemoryCache<A, B> {

-   fn has_key(&self, key: &A) -> bool
+   fn contains_key(&self, key: &A) -> bool

-   fn set(&mut self, key: A, value: B, duration: Option<Duration>) -> Option<B>
+   fn insert(&mut self, key: A, value: B, lifetime: Option<Duration>) -> Option<B>

-   fn get_or_set<F>(&mut self, key: A, factory: F, duration: Option<Duration>) -> &B
+   fn get_or_insert<F>(&mut self, key: A, factory: F, lifetime: Option<Duration>) -> &B

}
```

**Changed types of parameters/results:**

```diff
MemoryCache<A, B> {

-   fn get_full_scan_frequency(&self) -> &Duration
+   fn get_full_scan_frequency(&self) -> Option<Duration>

}
```

Licence
===

[MIT](LICENSE)
