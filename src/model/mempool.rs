use std::collections::BTreeMap;

/// Priority mempool with a max capacity. Inserting new items will drop the
/// item with the smallest key if it exceeds the max capacity.
#[derive(Debug, Clone)]
pub struct Mempool<K, V> {
    /// The data is backed by a BTreeMap, which makes it efficient to both get
    /// the largest and smallest values. It may be more optimal to use
    /// something like a binary heap due to constant time O(1) insertions, but
    /// it would only provide a single max/min value while we need both.
    pub data: BTreeMap<K, V>,
    /// Max size of mempool. The value with the smallest key will be removed if
    /// inserting a new item will exceed this size.
    pub max_size: Option<usize>,
}

impl<K: Ord + Clone, V> Mempool<K, V> {
    /// Create a new mempool with an unbound size.
    pub fn new() -> Self {
        Mempool {
            data: BTreeMap::new(),
            max_size: None,
        }
    }

    /// Creates a new mempool with a provided maximum size.
    pub fn new_with_capacity(max_size: usize) -> Self {
        Mempool {
            data: BTreeMap::new(),
            max_size: Some(max_size),
        }
    }

    /// Adds a new item. This will drop the smallest item if the additional item
    /// will cause the mempool to exceed the max size.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(max_size) = self.max_size {
            if self.data.len() >= max_size {
                // Smallest values are at "front", biggest at "end"
                let smallest_key = self.data.keys().next().cloned();

                if let Some(smallest_key) = smallest_key {
                    // Only remove the key if it does **not** exist in the pool
                    // This prevents unnecessarily removing an item when there
                    // is a duplicate key.
                    if !self.data.contains_key(&key) {
                        self.data.remove(&smallest_key);
                    }
                }
            }
        }

        self.data.insert(key, value)
    }

    /// Remove and returns the value with the largest key.
    pub fn pop(&mut self) -> Option<V> {
        let key = self.data.keys().next().cloned();
        key.and_then(|k| self.data.remove(&k))
    }

    /// Gets the value with the largest key in the mempool.
    pub fn max_value(&self) -> Option<&V> {
        self.data.values().next_back()
    }

    /// Gets the largest key in the mempool.
    pub fn max_key(&self) -> Option<&K> {
        self.data.keys().next_back()
    }

    /// Gets the value with the smallest key in the mempool.
    pub fn min_value(&self) -> Option<&V> {
        self.data.values().next()
    }

    /// Gets the smallest key in the mempool.
    pub fn min_key(&self) -> Option<&K> {
        self.data.keys().next()
    }

    /// Gets the current size of the mempool.
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gets_max() {
        let mut pool: Mempool<u64, ()> = Mempool::new_with_capacity(10);
        pool.insert(1, ());
        pool.insert(2, ());
        pool.insert(4, ());
        pool.insert(8, ());

        assert_eq!(pool.max_key(), Some(&8));
    }

    #[test]
    fn gets_min() {
        let mut pool: Mempool<u64, ()> = Mempool::new_with_capacity(10);
        pool.insert(1, ());
        pool.insert(2, ());
        pool.insert(4, ());
        pool.insert(8, ());

        assert_eq!(pool.min_key(), Some(&1));
    }

    #[test]
    fn drops_smallest_at_max() {
        let pool_size = 6;

        let mut pool: Mempool<u64, ()> = Mempool::new_with_capacity(pool_size);
        pool.insert(1, ());
        pool.insert(2, ());
        pool.insert(4, ());
        pool.insert(8, ());
        pool.insert(16, ());
        pool.insert(32, ());

        assert_eq!(pool.min_key(), Some(&1));

        // Max size exceeded, "1" will be removed.
        pool.insert(64, ());

        assert_eq!(pool.min_key(), Some(&2));
        assert_eq!(pool.len(), pool_size);

        // Additional inserts stay at max size
        for x in 0..100 {
            pool.insert(x, ());
            assert_eq!(pool.len(), pool_size);
        }
    }
}
