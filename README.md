[memory-cache-rs](https://docs.rs/memory-cache-rs)
===

Simple local in-memory cache for [Rust](https://www.rust-lang.org/).

Example
---

```rust
use memory_cache::MemoryCache;
use std::time::Duration;

// Full scan frequency to discover the expired entries.
let scan_frequency = Duration::from_secs(60);

let mut cache = MemoryCache::new(scan_frequency);

let key: &'static str = "key";
let value: &'static str = "Hello, World!";

// `None` - if the value must be kept forever.
let key_expiration = Some(Duration::from_secs(30));

cache.set(key, value, key_expiration);

assert_eq!(cache.get(&key), Some(&value));
```

Licence
===

[MIT](LICENSE)
