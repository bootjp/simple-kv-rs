use std::collections::HashMap;

pub struct KVStore {
    map: HashMap<Vec<u8>, Vec<u8>>,
}

trait KVService {
    fn new() -> Self;
    fn put(&mut self, key: &[u8], value: &[u8]);
    fn get(&self, key: &[u8]) -> Option<&[u8]>;
    fn delete(&mut self, key: &[u8]);
}

impl KVService for KVStore {
    fn new() -> Self {
        KVStore {
            map: HashMap::new(),
        }
    }

    fn put(&mut self, key: &[u8], value: &[u8]) {
        self.map.insert(Vec::from(key), Vec::from(value));
    }

    fn get(&self, key: &[u8]) -> Option<&[u8]> {
        self.map.get(key).map(|v| v.as_slice())
    }

    fn delete(&mut self, key: &[u8]) {
        self.map.remove(&Vec::from(key));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put() {
        let mut store = KVStore::new();
        store.put(b"key", b"value");
        assert_eq!(store.get(b"key"), Some(b"value".as_slice()));
    }

    #[test]
    fn test_get() {
        let mut store = KVStore::new();
        store.put(b"key", b"value");
        assert_eq!(store.get(b"key"), Some(b"value".as_slice()));
    }

    #[test]
    fn test_delete() {
        let mut store = KVStore::new();
        store.put(b"key", b"value");
        store.delete(b"key");
        assert_eq!(store.get(b"key"), None);
    }
}
