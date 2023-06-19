use std::collections::HashMap;

/// Key-Value Store with in-memory storage.
pub struct MemoryStore {
    data: HashMap<String, Vec<u8>>,
}

impl MemoryStore {
    /// Create a new instance of MemoryStore.
    pub fn new() -> Self {
        MemoryStore { data: HashMap::new() }
    }

    /// Insert a key-value pair into the store.
    pub fn insert(&mut self, key: String, value: Vec<u8>) {
        self.data.insert(key, value);
    }

    /// Retrieves the value associated with the given key.
    pub fn retrieve(&self, key: &str) -> Option<&Vec<u8>> {
        self.data.get(key)
    }

    /// Deletes the key-value pair associated with the given key.
    pub fn delete(&mut self, key: &str) -> Option<Vec<u8>> {
        self.data.remove(key)
    }
}

