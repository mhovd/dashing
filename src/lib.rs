use std::{hash::Hash, sync::atomic::AtomicUsize};

pub mod lru;
pub mod unbounded;

pub trait Cache<K, V>
where
    K: Eq + Hash,
{
    fn insert(&self, key: K, value: V);
    fn get(&self, key: &K) -> Option<V>;
    fn remove(&self, key: &K) -> Option<V>;
    fn clear(&self);
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn hits(&self) -> usize;
    fn misses(&self) -> usize;
}

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
