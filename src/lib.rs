use dashmap::DashMap;
use std::hash::Hash;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// A thread-safe cache with built-in statistics for hits and misses.
///
/// The cache is based on DashMap and can be shared across multiple threads.
/// It automatically handles synchronization and allows safe concurrent access.
pub struct Cache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    inner: Arc<CacheInner<K, V>>,
}

struct CacheInner<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    map: DashMap<K, V>,
    hits: AtomicUsize,
    misses: AtomicUsize,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    /// Creates a new empty cache.
    pub fn new() -> Self {
        Cache {
            inner: Arc::new(CacheInner {
                map: DashMap::new(),
                hits: AtomicUsize::new(0),
                misses: AtomicUsize::new(0),
            }),
        }
    }

    /// Inserts a key-value pair into the cache.
    pub fn insert(&self, key: K, value: V) {
        self.inner.map.insert(key, value);
    }

    /// Gets a value from the cache, if it exists.
    ///
    /// Increments the hit counter if the key is found, otherwise increments the miss counter.
    pub fn get(&self, key: &K) -> Option<V> {
        if let Some(value) = self.inner.map.get(key) {
            self.inner.hits.fetch_add(1, Ordering::SeqCst);
            Some(value.clone())
        } else {
            self.inner.misses.fetch_add(1, Ordering::SeqCst);
            None
        }
    }

    /// Removes a key-value pair from the cache, returning the value if it existed.
    pub fn remove(&self, key: &K) -> Option<V> {
        self.inner.map.remove(key).map(|(_, v)| v)
    }

    /// Clears the cache.
    pub fn clear(&self) {
        self.inner.map.clear();
    }

    /// Returns the number of entries in the cache.
    pub fn len(&self) -> usize {
        self.inner.map.len()
    }

    /// Returns true if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.map.is_empty()
    }

    /// Returns the number of hits.
    pub fn hits(&self) -> usize {
        self.inner.hits.load(Ordering::SeqCst)
    }

    /// Returns the number of misses.
    pub fn misses(&self) -> usize {
        self.inner.misses.load(Ordering::SeqCst)
    }
}

impl<K, V> Clone for Cache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Cache {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<K, V> Default for Cache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let cache = Cache::new();
        cache.insert(1, "a");
        assert_eq!(cache.get(&1), Some("a"));
        assert_eq!(cache.get(&2), None);
    }

    #[test]
    fn test_remove() {
        let cache = Cache::new();
        cache.insert(1, "a");
        assert_eq!(cache.remove(&1), Some("a"));
        assert_eq!(cache.remove(&1), None);
    }

    #[test]
    fn test_hits_and_misses() {
        let cache = Cache::new();
        cache.insert(1, "a");

        // Access the cache to generate hits and misses
        assert_eq!(cache.get(&1), Some("a")); // hit
        assert_eq!(cache.get(&2), None); // miss

        assert_eq!(cache.hits(), 1);
        assert_eq!(cache.misses(), 1);
    }

    #[test]
    fn test_clear() {
        let cache = Cache::new();
        cache.insert(1, "a");
        cache.clear();
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_multithreaded() {
        let cache = Cache::new();
        let mut handles = vec![];

        for i in 0..10 {
            let cache_clone = cache.clone();
            let handle = std::thread::spawn(move || {
                cache_clone.insert(i, i * 2);
                assert_eq!(cache_clone.get(&i), Some(i * 2));
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Verify that all values are present
        for i in 0..10 {
            assert_eq!(cache.get(&i), Some(i * 2));
        }
    }
}
