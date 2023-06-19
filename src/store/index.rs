use std::collections::HashMap;
use std::io;

/// Key-Value Store with indexing support.
pub struct IndexedStore<S> {
    store: S,
    index: HashMap<String, u64>,
}

impl<S> IndexedStore<S>
where
    S: KeyValueStore,
{
    /// Creates a new instance of IndexedStore using the specified underlying key-value store.
    pub fn new(store: S) -> Self {
        IndexedStore {
            store,
            index: HashMap::new(),
        }
    }

    /// Inserts a key-value pair into the indexed store.
    pub fn insert(&mut self, key: String, value: Vec<u8>) -> io::Result<()> {
        self.store.insert(key.clone(), value.clone())?;
        self.index.insert(key, value.len() as u64);
        Ok(())
    }

    /// Retrieves the value associated with the given key from the indexed store.
    pub fn retrieve(&self, key: &str) -> io::Result<Option<Vec<u8>>> {
        self.store.retrieve(key)
    }

    /// Deletes the key-value pair associated with the given key from the indexed store.
    pub fn delete(&mut self, key: &str) ->  io::Result<Option<Vec<u8>>> {
        self.index.remove(key);
        self.store.delete(key)
    }
}


// Trait representing a key-value store.
pub trait KeyValueStore {
    /// Inserts a key-value pair into the store
    fn insert(&mut self, key: String, value: Vec<u8>) -> io::Result<()>;

    /// Retrieves the value associated with the given key from the store.
    fn retrieve(&self, key: &str) -> io::Result<Option<Vec<u8>>>;


    /// Deletes the key-value pair associated with the given key from the store.
    fn delete(&mut self, key: &str) ->  io::Result<Option<Vec<u8>>>;
}