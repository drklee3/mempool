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
    pub max_size: Option<u64>,
}

impl<K: Ord + Clone, V> Mempool<K, V> {
    pub fn new() -> Self {
        Mempool {
            data: BTreeMap::new(),
            max_size: None,
        }
    }

    pub fn max_size(&mut self, max_size: u64) -> Self {
        self.max_size.replace(max_size)
    }

    pub fn new_with_capacity(max_size: u64) -> Self {
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
                    self.data.remove(&smallest_key);
                }
            }
        }

        self.data.insert(key, value)
    }
}
