use dashmap::DashMap;
use std::hash::Hash;
use std::sync::Arc;

use crate::{Cache, Statistics};

/// An unbounded cache that stores key-value pairs in a `DashMap`.
pub struct Unbounded<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    inner: Arc<UnboundedInner<K, V>>,
}

struct UnboundedInner<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    map: DashMap<K, V>,
    statistics: Statistics,
}

impl<K, V> Unbounded<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    /// Creates a new unbounded cache.
    pub fn new() -> Self {
        Unbounded {
            inner: Arc::new(UnboundedInner {
                map: DashMap::new(),
                statistics: Statistics::new(),
            }),
        }
    }
}

impl<K, V> Cache<K, V> for Unbounded<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    /// Inserts a key-value pair into the cache.
    fn insert(&self, key: K, value: V) {
        self.inner.map.insert(key, value);
    }

    fn get(&self, key: &K) -> Option<V> {
        if let Some(value) = self.inner.map.get(key) {
            self.inner.statistics.add_hit();
            Some(value.clone())
        } else {
            self.inner.statistics.add_miss();
            None
        }
    }

    fn remove(&self, key: &K) -> Option<V> {
        self.inner.map.remove(key).map(|(_, v)| v)
    }

    fn clear(&self) {
        self.inner.map.clear();
    }

    fn len(&self) -> usize {
        self.inner.map.len()
    }

    fn is_empty(&self) -> bool {
        self.inner.map.is_empty()
    }

    fn hits(&self) -> usize {
        self.inner.statistics.hits()
    }

    fn misses(&self) -> usize {
        self.inner.statistics.misses()
    }
}

impl<K, V> Clone for Unbounded<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Unbounded {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<K, V> Default for Unbounded<K, V>
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
        let cache = Unbounded::new();
        cache.insert(1, "a");
        assert_eq!(cache.get(&1), Some("a"));
        assert_eq!(cache.get(&2), None);
    }

    #[test]
    fn test_remove() {
        let cache = Unbounded::new();
        cache.insert(1, "a");
        assert_eq!(cache.remove(&1), Some("a"));
        assert_eq!(cache.remove(&1), None);
    }

    #[test]
    fn test_hits_and_misses() {
        let cache = Unbounded::new();
        cache.insert(1, "a");

        // Access the cache to generate hits and misses
        assert_eq!(cache.get(&1), Some("a")); // hit
        assert_eq!(cache.get(&2), None); // miss

        assert_eq!(cache.hits(), 1);
        assert_eq!(cache.misses(), 1);
    }

    #[test]
    fn test_clear() {
        let cache = Unbounded::new();
        cache.insert(1, "a");
        cache.clear();
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_multithreaded() {
        let cache = Unbounded::new();
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
