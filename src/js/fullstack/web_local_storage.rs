pub struct WebLocalStorage {
    #[cfg(not(feature = "server"))]
    storage: web_sys::Storage,
}

impl WebLocalStorage {
    #[cfg(feature = "server")]
    pub fn new() -> Self {
        Self {}
    }
    pub fn get(&self, _key: &str) -> Option<String> {
        #[cfg(not(feature = "server"))]
        return self.storage.get(_key).unwrap();

        #[cfg(feature = "server")]
        return None;
    }

    pub fn set(&self, _key: &str, _value: &str) {
        #[cfg(not(feature = "server"))]
        self.storage.set(_key, _value).unwrap();
    }
}

#[cfg(not(feature = "server"))]
impl From<web_sys::Storage> for WebLocalStorage {
    fn from(storage: web_sys::Storage) -> Self {
        Self { storage }
    }
}
