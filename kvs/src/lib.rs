#![deny(missing_docs)]

//! # KvStore
//!
//! `kvs` is distributed key value store

use std::collections::HashMap;

/// Stores in memory key value pair
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// Creates a new instance of KvStore
    /// ```rust
    /// # use kvs::KvStore;
    /// let store = KvStore::new();
    /// ```
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    /// Gets the Option of value for the specified key.
    /// ```rust
    /// # use kvs::KvStore;
    /// # let store = KvStore::new();
    /// let value = store.get("key1".to_owned());
    /// ```
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    /// Sets key value pair.
    /// ```rust
    /// # use kvs::KvStore;
    /// # let mut store = KvStore::new();
    /// store.set("key1".to_owned(), "value".to_owned());
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// Removes the value for the specified key
    /// ```rust
    /// # use kvs::KvStore;
    /// # let mut store = KvStore::new();
    /// store.remove("key".to_owned())
    /// ```
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
