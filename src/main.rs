use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

pub struct KVStore {
    map: HashMap<String, String>,
}

trait KVService {
    fn new() -> Self;
    fn put(&mut self, key: Vec<u8>, value: Vec<u8>);
    fn get(&self, key: Vec<u8>) -> Option<Vec<u8>>;
    fn delete(&mut self, key: Vec<u8>);
}

impl KVService for KVStore {
    fn new() -> Self {
        KVStore {
            map: HashMap::new(),
        }
    }

    fn put(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.map.insert(String::from_utf8(key).unwrap(), String::from_utf8(value).unwrap());
    }

    fn get(&self, key: Vec<u8>) -> Option<Vec<u8>> {
        self.map.get(&String::from_utf8(key).unwrap()).map(|v| v.as_bytes().to_vec())
    }

    fn delete(&mut self, key: Vec<u8>) {
        self.map.remove(&String::from_utf8(key).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put() {
        let mut store = KVStore::new();
        store.put("key".as_bytes().to_vec(), "value".as_bytes().to_vec());
        assert_eq!(store.get("key".as_bytes().to_vec()), Some("value".as_bytes().to_vec()));
    }

    #[test]
    fn test_get() {
        let mut store = KVStore::new();
        store.put("key".as_bytes().to_vec(), "value".as_bytes().to_vec());
        assert_eq!(store.get("key".as_bytes().to_vec()), Some("value".as_bytes().to_vec()));
    }

    #[test]
    fn test_delete() {
        let mut store = KVStore::new();
        store.put("key".as_bytes().to_vec(), "value".as_bytes().to_vec());
        store.delete("key".as_bytes().to_vec());
        assert_eq!(store.get("key".as_bytes().to_vec()), None);
    }
}
