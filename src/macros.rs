/// Macro for defining functions whose result needs to be cached.
///
/// # Example
/// ```
/// use once_cell::sync::Lazy;
/// use std::sync::Mutex;
/// use memory_cache::{MemoryCache, cached};
///
/// cached! {
///     fn factorial(x: u128) -> u128 = {
///         if x <= 1 {
///             1
///         } else {
///             x * factorial(x - 1)
///         }
///     }
/// }
///
/// assert_eq!(factorial(21), 51090942171709440000);
/// ```
#[macro_export]
macro_rules! cached {
    (fn $name:ident ($($arg:ident: $arg_type:ty), *) -> $ret:ty = $body:expr) => {
        #[allow(unused_parens)]
        fn $name($($arg: $arg_type), *) -> $ret {
            // Static instance of `MemoryCache<A, B>`.
            static CACHE: Lazy<Mutex<MemoryCache<($($arg_type),*), $ret>>> =
                Lazy::new(|| Mutex::new(MemoryCache::new()));

            // Create key.
            let key = ($($arg.clone()), *);

            // Acquires a mutex for check cached value.
            let cache = CACHE.lock().unwrap();

            match cache.get(&key) {
                Some(value) => value.clone(),
                None => {
                    // Dispose mutex before executing body expression,
                    // to avoid deadlock during a recursive call.
                    drop(cache);

                    // Execute the body expression.
                    let value = (||$body)();

                    // Acquires a mutex for add/update cache.
                    let mut cache = CACHE.lock().unwrap();
                    cache.insert(key, value, None);
                    value.clone()
                }
            }
        }
    };
}
