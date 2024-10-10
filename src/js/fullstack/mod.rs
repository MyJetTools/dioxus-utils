#[cfg(not(feature = "server"))]
mod set_focus;
#[cfg(not(feature = "server"))]
pub use set_focus::*;

#[cfg(feature = "server")]
mod set_focus_server_mock;
#[cfg(feature = "server")]
pub use set_focus_server_mock::*;

mod web_local_storage;
pub use web_local_storage::*;

mod eval;
pub use eval::*;
mod now_date_time;
pub use now_date_time::*;

mod console_log;
pub use console_log::*;

mod reload_page;
pub use reload_page::*;
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
        #[cfg(not(feature = "server"))]
        return GlobalAppSettings::get_window()
            .local_storage()
            .unwrap()
            .unwrap()
            .into();

        #[cfg(feature = "server")]
        return super::WebLocalStorage::new();
    }
}
