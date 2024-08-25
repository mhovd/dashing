use dashmap::DashMap;
use std::collections::VecDeque;
use std::hash::Hash;
use std::sync::{Arc, Mutex};

use crate::{Cache, Statistics};

/// An LRU cache that stores key-value pairs in a `DashMap`.
pub struct LRUCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    inner: Arc<LRUCacheInner<K, V>>,
}

struct LRUCacheInner<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    map: DashMap<K, V>,
    order: Mutex<VecDeque<K>>,
    capacity: usize,
    statistics: Statistics,
}

impl<K, V> LRUCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    /// Creates a new LRUCache with the specified capacity.
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            inner: Arc::new(LRUCacheInner {
                map: DashMap::new(),
                order: Mutex::new(VecDeque::new()),
                capacity,
                statistics: Statistics::new(),
            }),
        }
    }

    fn evict_if_needed(&self) {
        let oldest_key = {
            let mut order = self.inner.order.lock().unwrap();
            if order.len() > self.inner.capacity {
                order.pop_front()
            } else {
                None
            }
        };

        if let Some(key) = oldest_key {
            self.inner.map.remove(&key);
        }
    }

    fn update_order(&self, key: K) {
        let mut order = self.inner.order.lock().unwrap();
        if let Some(pos) = order.iter().position(|k| *k == key) {
            order.remove(pos);
        }
        order.push_back(key);
    }
}

impl<K, V> Cache<K, V> for LRUCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn insert(&self, key: K, value: V) {
        self.inner.map.insert(key.clone(), value);
        self.update_order(key);
        self.evict_if_needed();
    }

    fn get(&self, key: &K) -> Option<V> {
        if let Some(value) = self.inner.map.get(key) {
            self.update_order(key.clone());
            self.inner.statistics.add_hit();
            Some(value.clone())
        } else {
            self.inner.statistics.add_miss();
            None
        }
    }

    fn remove(&self, key: &K) -> Option<V> {
        if let Some(value) = self.inner.map.remove(key) {
            let mut order = self.inner.order.lock().unwrap();
            if let Some(pos) = order.iter().position(|k| k == key) {
                order.remove(pos);
            }
            Some(value.1)
        } else {
            None
        }
    }

    fn clear(&self) {
        self.inner.map.clear();
        let mut order = self.inner.order.lock().unwrap();
        order.clear();
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

impl<K, V> Clone for LRUCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        LRUCache {
            inner: Arc::clone(&self.inner),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let cache = LRUCache::new(3);
        cache.insert(1, "one");
        cache.insert(2, "two");
        cache.insert(3, "three");

        assert_eq!(cache.get(&1), Some("one"));
        assert_eq!(cache.get(&2), Some("two"));
        assert_eq!(cache.get(&3), Some("three"));
    }

    #[test]
    fn test_insert_evict_and_get() {
        let cache = LRUCache::new(3);
        cache.insert(1, "one");
        cache.insert(2, "two");
        cache.insert(3, "three");
        cache.insert(4, "four");

        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some("two"));
        assert_eq!(cache.get(&3), Some("three"));
        assert_eq!(cache.get(&4), Some("four"));
    }

    #[test]
    fn test_remove() {
        let cache = LRUCache::new(3);
        cache.insert(1, "one");
        cache.insert(2, "two");
        cache.insert(3, "three");

        assert_eq!(cache.remove(&2), Some("two"));
        assert_eq!(cache.get(&2), None);
    }

    #[test]
    fn test_clear() {
        let cache = LRUCache::new(3);
        cache.insert(1, "one");
        cache.insert(2, "two");
        cache.insert(3, "three");

        cache.clear();

        assert_eq!(cache.len(), 0);
        assert_eq!(cache.is_empty(), true);
    }

    #[test]
    fn test_hits_and_misses() {
        let cache = LRUCache::new(3);
        cache.insert(1, "one");
        cache.insert(2, "two");
        cache.insert(3, "three");

        assert_eq!(cache.hits(), 0);
        assert_eq!(cache.misses(), 0);

        cache.get(&1);
        cache.get(&2);
        cache.get(&4);

        assert_eq!(cache.hits(), 2);
        assert_eq!(cache.misses(), 1);
    }

    #[test]
    fn test_multithreaded() {
        let cache = LRUCache::new(5);
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

        // Verify the cache size
        assert_eq!(cache.len(), 5, "Cache size is {}", cache.len());
    }
}
