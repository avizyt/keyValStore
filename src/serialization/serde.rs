use serde::{Serialize, Deserialize};
use bincode::{serialize as bincode_serialize, deserialize as bincode_deserialize};

pub fn serialize<T: Serialize>(value: &T) -> Vec<u8> {
    bincode_serialize(value).unwrap()
}

pub fn deserialize<'a, T: Deserialize<'a>>(data: &'a[u8]) -> T {
    bincode_deserialize(data).unwrap()
}