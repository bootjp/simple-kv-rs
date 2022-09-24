use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

pub struct KVStore {
    map: HashMap<Vec<u8>, Vec<u8>>,
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
        self.map.insert(key, value);
    }

    fn get(&self, key: Vec<u8>) -> Option<Vec<u8>> {
        self.map.get(&key).map(|v| v.clone())
    }

    fn delete(&mut self, key: Vec<u8>) {
        self.map.remove(&key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put() {
        let mut store = KVStore::new();
        store.put(b"key".to_vec(), b"value".to_vec());
        assert_eq!(store.get(b"key".to_vec()), Some(b"value".to_vec()));
    }

    #[test]
    fn test_get() {
        let mut store = KVStore::new();
        store.put(b"key".to_vec(), b"value".to_vec());
        assert_eq!(store.get(b"key".to_vec()), Some(b"value".to_vec()));
        assert_eq!(store.get(b"key2".to_vec()), None);
    }

    #[test]
    fn test_delete() {
        let mut store = KVStore::new();
        store.put(b"key".to_vec(), b"value".to_vec());
        store.delete(b"key".to_vec());
        assert_eq!(store.get(b"key".to_vec()), None);
    }
}
