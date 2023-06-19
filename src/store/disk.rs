use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{PathBuf};
use crate::store::index::KeyValueStore;

// key-value store with dish storage.
pub struct DiskStore {
    data_dir: PathBuf,
}

impl DiskStore {
    /// Create new instance of DiskStore with
    pub fn new(data_dir: impl Into<PathBuf>) -> io::Result<Self> {
        let data_dir = data_dir.into();
        fs::create_dir_all(&data_dir)?;
        Ok(DiskStore { data_dir })
    }

    fn get_file_path(&self, key:&str) -> PathBuf{
        self.data_dir.join(key)
    }

    ///Insert a key-value pair
    pub fn insert(&self, key: String, value: Vec<u8>) -> io::Result<()> {
        let file_path = self.get_file_path(&key);
        let mut file = File::create(&file_path)?;
        file.write_all(&value)?;
        Ok(())
    }

    /// Retrieves the value associated with the given key from the disk store.
    pub fn retrieve(&self, key: &str) -> io::Result<Option<Vec<u8>>> {
        let file_path = self.get_file_path(key);
        if file_path.exists() {
            let mut file = File::open(&file_path)?;
            let mut value = Vec::new();
            file.read_to_end(&mut value)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    /// Deletes the key-value pair associated with the given key from the disk store.
    pub fn delete(&self, key: &str) -> io::Result<Option<Vec<u8>>> {
        let file_path = self.get_file_path(key);
        if file_path.exists() {
            let mut file = File::open(&file_path)?;
            let mut value = Vec::new();
            file.read_to_end(&mut value)?;
            fs::remove_file(&file_path)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

}

/// Trait representing a key-value store.
// pub trait KeyValueStore {
//     /// Inserts a key-value pair into the store.
//     fn insert(&self, key: String, value: Vec<u8>) -> io::Result<()>;

//     /// Retrieves the value associated with the given key from the store.
//     fn retrieve(&self, key: &str) -> io::Result<Option<Vec<u8>>>;

//     /// Deletes the key-value pair associated with the given key from the store.
//     fn delete(&self, key: &str) -> io::Result<Option<Vec<u8>>>;
// }

impl KeyValueStore for DiskStore {
    fn insert(&mut self, key: String, value: Vec<u8>) -> io::Result<()> {
        DiskStore::insert(self, key, value)
    }

    fn retrieve(&self, key: &str) -> io::Result<Option<Vec<u8>>> {
        DiskStore::retrieve(self, key)
    }

    fn delete(&mut self, key: &str) -> io::Result<Option<Vec<u8>>> {
        DiskStore::delete(self, key)
    }
}