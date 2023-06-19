// use key_value_store::store::memory::MemoryStore;
use key_value_store::store::disk::DiskStore;
use key_value_store::store::index::{IndexedStore};
// use key_value_store::store::index::KeyValueStore;

fn main() {
    // Create an instance of the disk-based key-value store
    let disk_store = DiskStore::new("./data").expect("Failed to create disk store");

    // Create an instance of the indexed key-value store
    let mut indexed_store = IndexedStore::new(disk_store);

    // Insert key-value pairs
    indexed_store.insert("key1".to_string(), vec![1, 2, 3]).expect("Failed to insert key1");
    indexed_store.insert("key2".to_string(), vec![4, 5, 6]).expect("Failed to insert key2");

    // Retrieve values
    let value1 = indexed_store.retrieve("key1").expect("Failed to retrieve key1");
    let value2 = indexed_store.retrieve("key2").expect("Failed to retrieve key2");

    match value1 {
        Some(value) => println!("Value for key1: {:?}", value),
        None => println!("Key1 not found"),
    }

    match value2 {
        Some(value) => println!("Value for key2: {:?}", value),
        None => println!("Key2 not found"),
    }

    // Delete a key-value pair
    let deleted_value = indexed_store.delete("key1").expect("Failed to delete key1");

    match deleted_value {
        Some(value) => println!("Deleted value for key1: {:?}", value),
        None => println!("Key1 not found for deletion"),
    }

    println!("----------------------------");

    println!("All operation are completed!!😃")
}
