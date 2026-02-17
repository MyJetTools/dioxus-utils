pub struct WebLocalStorage {
    storage: web_sys::Storage,
}

impl WebLocalStorage {
    pub fn get(&self, _key: &str) -> Option<String> {
        return self.storage.get(_key).unwrap();
    }

    pub fn set(&self, _key: &str, _value: &str) {
        #[cfg(not(feature = "server"))]
        self.storage.set(_key, _value).unwrap();
    }

    pub fn delete(&self, _key: &str) {
        #[cfg(not(feature = "server"))]
        self.storage.delete(_key).unwrap();
    }
}

impl From<web_sys::Storage> for WebLocalStorage {
    fn from(storage: web_sys::Storage) -> Self {
        Self { storage }
    }
}
