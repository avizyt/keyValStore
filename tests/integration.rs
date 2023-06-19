use key_value_store::store::disk::DiskStore;
use key_value_store::store::index::{IndexedStore};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_key_value_store() {
        // Create an instance of the disk-based key-value store
        let disk_store = DiskStore::new("./test_data").expect("Failed to create disk store");

        // Create an instance of the indexed key-value store
        let mut indexed_store = IndexedStore::new(disk_store);

        // Insert key-value pairs
        indexed_store.insert("key1".to_string(), vec![1, 2, 3]).expect("Failed to insert key1");
        indexed_store.insert("key2".to_string(), vec![4, 5, 6]).expect("Failed to insert key2");

        // Retrieve values
        let value1 = indexed_store.retrieve("key1").expect("Failed to retrieve key1");
        let value2 = indexed_store.retrieve("key2").expect("Failed to retrieve key2");

        assert_eq!(value1, Some(vec![1, 2, 3]));
        assert_eq!(value2, Some(vec![4, 5, 6]));

        // Delete a key-value pair
        let deleted_value = indexed_store.delete("key1").expect("Failed to delete key1");

        assert_eq!(deleted_value, Some(vec![1, 2, 3]));

        // Verify deletion
        let deleted_value = indexed_store.retrieve("key1").expect("Failed to retrieve key1 after deletion");
        assert_eq!(deleted_value, None);
    }
}
