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