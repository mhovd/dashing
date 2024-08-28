use anyhow::Result;
use std::{hash::Hash, sync::atomic::AtomicUsize};
pub mod lru;
pub mod unbounded;

pub trait Cache<K, V>
where
    K: Eq + Hash,
{
    /// Inserts a key-value pair into the cache.
    fn insert(&self, key: K, value: V);
    /// Returns the value associated with the key.
    fn get(&self, key: &K) -> Option<V>;
    /// Removes the value associated with the key.
    fn remove(&self, key: &K) -> Option<V>;
    /// Removes all key-value pairs from the cache.
    fn clear(&self);
    /// Returns the number of key-value pairs in the cache.
    fn len(&self) -> usize;
    /// Returns `true` if the cache is empty.
    fn is_empty(&self) -> bool;
    /// Returns the number of cache hits.
    fn hits(&self) -> usize;
    /// Returns the number of cache misses.
    fn misses(&self) -> usize;
    /// Write the cache to a file.
    fn write_to_file(&self, file_name: &str) -> Result<()>;
    /// Read the cache from a file.
    fn read_from_file(&self, file_name: &str) -> Result<()>;
}

/// A struct that holds statistics about cache hits and misses.
struct Statistics {
    hits: AtomicUsize,
    misses: AtomicUsize,
}

impl Statistics {
    fn new() -> Self {
        Statistics {
            hits: AtomicUsize::new(0),
            misses: AtomicUsize::new(0),
        }
    }

    fn hits(&self) -> usize {
        self.hits.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn misses(&self) -> usize {
        self.misses.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn add_hit(&self) {
        self.hits.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    fn add_miss(&self) {
        self.misses
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}
