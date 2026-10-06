/// Writes every panic to the browser console as an error `panic: <text>`. Call it once in `main()`.
///
/// A debug build of Dioxus installs its own panic hook at launch, which replaces this one.
#[cfg(not(feature = "server"))]
pub fn set_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let text = info.payload_as_str().unwrap_or("Box<dyn Any>");
        let message = match info.location() {
            Some(location) => format!("panic: {}\n    at {}", text, location),
            None => format!("panic: {}", text),
        };

        crate::console_error(message);
    }));
}

/// On the server a panic is already printed to stderr.
#[cfg(feature = "server")]
pub fn set_panic_hook() {}
