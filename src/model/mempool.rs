use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Mempool<K, V> {
    /// Mempool is backed by a BTreeMap, which makes it efficient to both get
    /// the largest and smallest values. It may be more optimal to use
    /// something like a binary heap due to constant time O(1) insertions, but
    /// it would only provide a single max/min value while we need both.
    pub data: BTreeMap<K, V>,
    /// Max size of mempool. The value with the smallest key will be removed if
    /// inserting a new item will exceed this size.
    pub max_size: Option<usize>,
}

impl<K: Ord + Clone, V> Mempool<K, V> {
    pub fn new() -> Self {
        Mempool {
            data: BTreeMap::new(),
            max_size: None,
        }
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        Mempool {
            data: BTreeMap::new(),
            max_size: Some(capacity),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // If inserting will exceed max size, remove the smallest element first
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
