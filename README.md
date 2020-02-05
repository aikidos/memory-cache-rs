[memory-cache-rs](https://docs.rs/memory-cache-rs)
===

Simple local in-memory cache for [Rust](https://www.rust-lang.org/).

[Breaking Changes](BREAKING.md)

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

Licence
===

[MIT](LICENSE)
