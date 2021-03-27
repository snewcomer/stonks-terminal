use std::collections::HashMap;

pub trait Store {
    fn get(&self, key: String) -> Option<&String>;
    fn put(&mut self, key: String, secret: String);
    fn delete(&mut self, key: String);
    fn set_verification_code(&mut self, verification_code: String);
}

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
}

impl Store for AuthInMemoryStore {
    fn get(&self, key: String) -> Option<&String> {
        self.data.get(&key)
    }

    fn put(&mut self, key: String, secret: String) {
        self.data.insert(key, secret);
    }

    fn delete(&mut self, key: String) {
        self.data.remove(&key);
    }

    fn set_verification_code(&mut self, verification_code: String) {
        self.verification_code = verification_code;
    }
}
