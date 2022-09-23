use std::collections::HashMap;


fn main() {
    println!("Hello, world!");
}

pub struct KVStore {
    map: HashMap<String, String>,
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
        self.map.insert(String::from_utf8(key.to_vec()).unwrap(), String::from_utf8(value.to_vec()).unwrap());
    }

    fn get(&self, key: &[u8]) -> Option<&[u8]> {
        self.map.get(&String::from_utf8(key.to_vec()).unwrap()).map(|s| s.as_bytes())
    }

    fn delete(&mut self, key: &[u8]) {
        self.map.remove(&String::from_utf8(key.to_vec()).unwrap());
    }
}