use std::thread::LocalKey;

use js_sys::wasm_bindgen::JsValue;

// web_sys::Storage is a JS object (!Send + !Sync), so it can not live in a `static` or in a LazyLock.
// thread_local! is the lazy static which works for it: the storage is resolved on first access
// and reused after that.
thread_local! {
    static LOCAL: web_sys::Storage = open("Local", web_sys::Window::local_storage);
    static SESSION: web_sys::Storage = open("Session", web_sys::Window::session_storage);
}

/// Browser `localStorage`. If it is not available - writes an error to the console and panics.
pub static LOCAL_STORAGE: WebStorage = WebStorage { storage: &LOCAL };

/// Browser `sessionStorage`. If it is not available - writes an error to the console and panics.
pub static SESSION_STORAGE: WebStorage = WebStorage { storage: &SESSION };

pub type WebLocalStorage = WebStorage;

#[derive(Clone, Copy)]
pub struct WebStorage {
    storage: &'static LocalKey<web_sys::Storage>,
}

impl WebStorage {
    pub fn get(&self, key: &str) -> Option<String> {
        self.storage.with(|storage| storage.get_item(key).unwrap())
    }

    pub fn set(&self, key: &str, value: &str) {
        self.storage
            .with(|storage| storage.set_item(key, value).unwrap());
    }

    pub fn delete(&self, key: &str) {
        self.storage
            .with(|storage| storage.remove_item(key).unwrap());
    }
}

fn open(
    name: &str,
    get_storage: fn(&web_sys::Window) -> Result<Option<web_sys::Storage>, JsValue>,
) -> web_sys::Storage {
    let reason = match web_sys::window() {
        Some(window) => match get_storage(&window) {
            Ok(Some(storage)) => return storage,
            Ok(None) => "browser does not provide it".to_string(),
            Err(err) => format!("browser denied access to it. Err: {:?}", err),
        },
        None => "no Js Window object returned".to_string(),
    };

    let message = format!("dioxus {} storage is not found: {}", name, reason);
    crate::console_error(message.as_str());
    panic!("{}", message);
}
