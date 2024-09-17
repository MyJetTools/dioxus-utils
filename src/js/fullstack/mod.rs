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
pub struct GlobalAppSettings {
    href: String,
}

impl GlobalAppSettings {
    pub fn get_window() -> web_sys::Window {
        web_sys::window().expect("No Js Window object returned")
    }

    pub fn get_href(&self) -> &str {
        &self.href
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
