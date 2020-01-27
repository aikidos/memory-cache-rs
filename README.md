memory-cache-rs
===

Simple local in-memory cache for [Rust](https://www.rust-lang.org/).

Example
---

```rust
use memory_cache::MemoryCache;
use std::time::Duration;

let scan_frequency = Duration::from_secs(60);

let mut cache = MemoryCache::new(scan_frequency);

let key: &'static str = "key";
let value: &'static str = "Hello, World!";
let expiration = Duration::from_secs(30);

cache.set(key, value, Some(expiration));

let cached_value = cache.get(&key);

assert_eq!(cached_value, Some(&value));
```

Licence
===

[MIT](LICENSE)
