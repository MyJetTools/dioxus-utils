mod sleep;
pub use sleep::*;
mod set_focus;
pub use set_focus::*;
mod reload_page;
pub use reload_page::*;
mod web_local_storage;
pub use web_local_storage::*;
mod fl_url;
pub use fl_url::*;

pub use url_utils::HttpRequestBody;
pub struct GlobalAppSettings {
    href: String,
    origin: String,
}

impl GlobalAppSettings {
    #[cfg(not(feature = "server"))]
    fn get_window() -> web_sys::Window {
        web_sys::window().expect("No Js Window object returned")
    }

    #[cfg(not(feature = "server"))]
    pub fn new() -> Self {
        let window = GlobalAppSettings::get_window();

        Self {
            href: window.location().href().unwrap(),
            origin: window.location().origin().unwrap(),
        }
    }

    #[cfg(feature = "server")]
    pub fn new() -> Self {
        Self {
            href: String::new(),
            origin: String::new(),
        }
    }

    pub fn get_href(&self) -> &str {
        &self.href
    }

    pub fn get_origin(&self) -> &str {
        &self.origin
    }

    pub fn get_local_storage() -> super::WebLocalStorage {
        return GlobalAppSettings::get_window()
            .local_storage()
            .unwrap()
            .unwrap()
            .into();
    }
}
