use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{hash::Hash, sync::atomic::AtomicUsize};
pub mod lru;
pub mod unbounded;

#[derive(Clone)]
pub enum Cache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static + Serialize + for<'a> Deserialize<'a>,
    V: Clone + Send + Sync + 'static + Serialize + for<'a> Deserialize<'a>,
{
    LRU(lru::LRU<K, V>),
    Unbounded(unbounded::Unbounded<K, V>),
    None,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static + Serialize + for<'a> Deserialize<'a>,
    V: Clone + Send + Sync + 'static + Serialize + for<'a> Deserialize<'a>,
{
    pub fn new_lru(capacity: usize) -> Self {
        Cache::LRU(lru::LRU::new(capacity))
    }

    pub fn new_unbounded() -> Self {
        Cache::Unbounded(unbounded::Unbounded::new())
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Cache::None)
    }

    pub fn is_some(&self) -> bool {
        !matches!(self, Cache::None)
    }

    pub fn insert(&self, key: K, value: V) {
        match self {
            Cache::LRU(cache) => cache.insert(key, value),
            Cache::Unbounded(cache) => cache.insert(key, value),
            Cache::None => {}
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        match self {
            Cache::LRU(cache) => cache.get(key),
            Cache::Unbounded(cache) => cache.get(key),
            Cache::None => None,
        }
    }

    pub fn remove(&self, key: &K) -> Option<V> {
        match self {
            Cache::LRU(cache) => cache.remove(key),
            Cache::Unbounded(cache) => cache.remove(key),
            Cache::None => None,
        }
    }

    pub fn clear(&self) {
        match self {
            Cache::LRU(cache) => cache.clear(),
            Cache::Unbounded(cache) => cache.clear(),
            Cache::None => {}
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Cache::LRU(cache) => cache.len(),
            Cache::Unbounded(cache) => cache.len(),
            Cache::None => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Cache::LRU(cache) => cache.is_empty(),
            Cache::Unbounded(cache) => cache.is_empty(),
            Cache::None => true,
        }
    }

    pub fn hits(&self) -> usize {
        match self {
            Cache::LRU(cache) => cache.hits(),
            Cache::Unbounded(cache) => cache.hits(),
            Cache::None => 0,
        }
    }

    pub fn misses(&self) -> usize {
        match self {
            Cache::LRU(cache) => cache.misses(),
            Cache::Unbounded(cache) => cache.misses(),
            Cache::None => 0,
        }
    }

    pub fn write(&self, file_name: &str) -> Result<()> {
        match self {
            Cache::LRU(cache) => cache.write(file_name),
            Cache::Unbounded(cache) => cache.write(file_name),
            Cache::None => Ok(()),
        }
    }

    pub fn read(&self, file_name: &str) -> Result<()> {
        match self {
            Cache::LRU(cache) => cache.read(file_name),
            Cache::Unbounded(cache) => cache.read(file_name),
            Cache::None => Ok(()),
        }
    }
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
