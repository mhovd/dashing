use dashmap::DashMap;
use std::hash::Hash;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// A thread-safe cache based on [DashMap](https://docs.rs/dashmap).
///
/// The cache stores key-value pairs and supports insertion, retrieval, and removal of entries.
///
/// # Examples
///
/// ```
/// use cache::Cache;
///
/// let cache = Cache::new();
/// cache.insert(1, 2);
///
/// assert_eq!(cache.get(&1), Some(2));
/// assert_eq!(cache.get(&2), None);
///
/// assert_eq!(cache.remove(&1), Some(2));
/// assert_eq!(cache.remove(&1), None);
///
/// cache.clear();
/// assert_eq!(cache.len(), 0);
/// ```
pub struct Cache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    map: Arc<DashMap<K, V>>,
    hits: Arc<AtomicUsize>,
    misses: Arc<AtomicUsize>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    /// Creates a new empty cache.
    pub fn new() -> Self {
        Cache {
            map: Arc::new(DashMap::new()),
            hits: Arc::new(AtomicUsize::new(0)),
            misses: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Creates a new empty cache with the specified initial capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Cache {
            map: Arc::new(DashMap::with_capacity(capacity)),
            hits: Arc::new(AtomicUsize::new(0)),
            misses: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Inserts a key-value pair into the cache.
    pub fn insert(&self, key: K, value: V) {
        self.map.insert(key, value);
    }

    /// Gets a value from the cache, if it exists.
    pub fn get(&self, key: &K) -> Option<V> {
        if let Some(value) = self.map.get(key) {
            self.hits.fetch_add(1, Ordering::SeqCst);
            Some(value.clone())
        } else {
            self.misses.fetch_add(1, Ordering::SeqCst);
            None
        }
    }

    /// Removes a key-value pair from the cache, returning the value if it existed.
    pub fn remove(&self, key: &K) -> Option<V> {
        self.map.remove(key).map(|(_, v)| v)
    }

    /// Clears the cache.
    pub fn clear(&self) {
        self.map.clear();
    }

    /// Returns the number of entries in the cache.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl Default for Cache<usize, usize> {
    fn default() -> Self {
        Cache::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let cache = Cache::new();
        cache.insert(1, 2);
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_get() {
        let cache = Cache::new();
        cache.insert(1, 2);
        assert_eq!(cache.get(&1), Some(2));
        assert_eq!(cache.get(&2), None);
    }

    #[test]
    fn test_remove() {
        let cache = Cache::new();
        cache.insert(1, 2);
        assert_eq!(cache.remove(&1), Some(2));
        assert_eq!(cache.remove(&1), None);
    }

    #[test]
    fn test_clear() {
        let cache = Cache::new();
        cache.insert(1, 2);
        cache.clear();
        assert_eq!(cache.len(), 0);
    }
}
