use crate::{Cache, Statistics};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::sync::Arc;

/// An unbounded cache that stores key-value pairs in a `DashMap`.
pub struct Unbounded<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static + Serialize + for<'a> Deserialize<'a>,
    V: Clone + Send + Sync + 'static + Serialize + for<'a> Deserialize<'a>,
{
    inner: Arc<UnboundedInner<K, V>>,
}

struct UnboundedInner<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static + Serialize,
    V: Clone + Send + Sync + 'static + Serialize,
{
    map: DashMap<K, V>,
    statistics: Statistics,
}

impl<K, V> Unbounded<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static + Serialize + for<'a> Deserialize<'a>,
    V: Clone + Send + Sync + 'static + Serialize + for<'a> Deserialize<'a>,
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
    K: Eq + Hash + Clone + Send + Sync + 'static + Serialize + for<'a> Deserialize<'a>,
    V: Clone + Send + Sync + 'static + Serialize + for<'a> Deserialize<'a>,
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

    fn write_to_file(&self, file_name: &str) {
        let _ = file_name;
        // Collect all entries from the dashmap
        let entries: Vec<(K, V)> = self
            .inner
            .map
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect();

        // Use bincode to serialize the entries
        let encoded: Vec<u8> = bincode::serialize(&entries).unwrap();

        // Write the encoded entries to a file
        std::fs::write(file_name, encoded).unwrap();
    }

    fn read_from_file(&self, file_name: &str) {
        // Read the encoded entries from a file
        let encoded = std::fs::read(file_name).unwrap();

        // Use bincode to deserialize the entries
        let entries: Vec<(K, V)> = bincode::deserialize(&encoded).unwrap();

        // Insert the entries into the dashmap
        for (key, value) in entries {
            self.inner.map.insert(key, value);
        }
    }
}

impl<K, V> Clone for Unbounded<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static + Serialize + for<'a> Deserialize<'a>,
    V: Clone + Send + Sync + 'static + Serialize + for<'a> Deserialize<'a>,
{
    fn clone(&self) -> Self {
        Unbounded {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<K, V> Default for Unbounded<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static + Serialize + for<'a> Deserialize<'a>,
    V: Clone + Send + Sync + 'static + Serialize + for<'a> Deserialize<'a>,
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
        cache.insert(1, "a".to_string());
        assert_eq!(cache.get(&1), Some("a".to_string()));
        assert_eq!(cache.get(&2), None);
    }

    #[test]
    fn test_remove() {
        let cache = Unbounded::new();
        cache.insert(1, "a".to_string());
        assert_eq!(cache.remove(&1), Some("a".to_string()));
        assert_eq!(cache.remove(&1), None);
    }

    #[test]
    fn test_hits_and_misses() {
        let cache = Unbounded::new();
        cache.insert(1, "a".to_string());

        // Access the cache to generate hits and misses
        assert_eq!(cache.get(&1), Some("a".to_string())); // hit
        assert_eq!(cache.get(&2), None); // miss

        assert_eq!(cache.hits(), 1);
        assert_eq!(cache.misses(), 1);
    }

    #[test]
    fn test_clear() {
        let cache = Unbounded::new();
        cache.insert(1, "a".to_string());
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
