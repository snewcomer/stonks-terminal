use std::collections::HashMap;

// in memory store
//
pub struct AuthInMemoryStore {
    data: HashMap<String, String>,
    pub verification_code: String,
}

impl AuthInMemoryStore {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            verification_code: "".to_string(),
        }
    }

    pub fn get(&mut self, key: String) {
        self.data.get(&key);
    }

    pub fn put(&mut self, key: String, secret: String) {
        self.data.insert(key, secret);
    }

    fn delete(&mut self, key: String) {
        self.data.remove(&key);
    }
}
