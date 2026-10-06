# dioxus-utils

A comprehensive utility library for Dioxus applications providing state management, browser APIs, and fullstack development utilities.

## Overview

`dioxus-utils` is a collection of utilities designed to simplify common tasks in Dioxus applications. It provides abstractions for data loading states, dialog/form management, browser interactions, and seamless client/server code sharing for fullstack applications.

## Features

- **State Management**: `DataState` and `RenderState` for managing async data loading states
- **Dialog Management**: `DialogValue` for tracking form changes in dialogs
- **Browser Utilities**: Console logging (`log`, `debug`, `info`, `warn`, `error`), a panic hook which makes panics visible in release builds, and JavaScript evaluation
- **Browser Storage**: `LOCAL_STORAGE` and `SESSION_STORAGE` objects - the way to access `localStorage` / `sessionStorage` in client-side (`web`) apps
- **Fullstack Support**: Client/server compatible utilities for focus management, local storage, page reload, and async sleep
- **Child Notification**: `NotifyChildComponent<TValue>` for delivering update events from parent to child components
- **Global Settings**: Access to window location through `GlobalAppSettings`

## Usage in this repo

This project uses a subset of `dioxus-utils` APIs. The examples below mirror the exact
patterns in the codebase to reduce drift.

### Data loading with `DataState` / `RenderState`

Best practice here is to keep loading logic in a `get_data` helper and keep the
component focused on rendering:

```rust
use dioxus::prelude::*;

#[component]
pub fn RenderSettingsPage() -> Element {
    let cs = use_signal(|| VadSettingsState::default());
    let cs_ra = cs.read();

    let input_data = match get_data(cs, &*cs_ra) {
        Ok(input_data) => input_data,
        Err(err) => return err,
    };

    render_vad_settings(cs, input_data)
}

fn get_data<'s>(
    mut cs: Signal<VadSettingsState>,
    cs_ra: &'s VadSettingsState,
) -> Result<&'s VadSettingsData, Element> {
    match cs_ra.data.as_ref() {
        dioxus_utils::RenderState::None => {
            let lang = cs_ra.lang;
            spawn(async move {
                cs.write().data.set_loading();
                let data = crate::api::vad_settings::get_vad_settings(lang).await;
                match data {
                    Ok(data) => cs.write().set_data(data),
                    Err(err) => cs.write().data.set_error(err.to_string()),
                }
            });
            Err(crate::components::loading())
        }
        dioxus_utils::RenderState::Loading => Err(crate::components::loading()),
        dioxus_utils::RenderState::Loaded(data) => Ok(data),
        dioxus_utils::RenderState::Error(err) => {
            Err(crate::components::loading_data_error(err))
        }
    }
}
```

After mutations (save/delete), the code resets the data so it reloads:

```rust
state.write().data.reset();
```

#### DataState helpers used here

Some pages use additional helpers beyond `set_value`:

```rust
// Mark data as loaded without changing the payload type
state.write().data.set_loaded(());

// Read only when loaded, otherwise fall back
let Some(data) = state.read().data.try_unwrap_as_loaded() else {
    return vec![];
};

// Guard validation until initial data arrives
if !state.read().data.has_value() {
    return false;
}
```

### Local and session storage via `LOCAL_STORAGE` / `SESSION_STORAGE`

Used for lightweight client-side persistence. In a client-side (`web`) app always go through
these two objects - do not take the storage from `web_sys::window()` yourself:

```rust
use dioxus_utils::js::LOCAL_STORAGE;

const STORAGE_KEY: &str = "client-view-search";

pub fn get() -> Vec<String> {
    let result = LOCAL_STORAGE.get(STORAGE_KEY);
    result
        .unwrap_or_default()
        .split(';')
        .map(|itm| itm.to_string())
        .collect()
}

pub fn save(items: &[String]) {
    let joined = items.join(";");
    LOCAL_STORAGE.set(STORAGE_KEY, joined.as_str());
}
```

`SESSION_STORAGE` has the same methods - see **Local and Session Storage** below.

### Background loops with `js::sleep`

Used to throttle refresh loops:

```rust
use dioxus_utils::js::sleep;
use std::time::Duration;

loop {
    refresh_data(cs).await;
    sleep(Duration::from_secs(3)).await;
}
```

### Console logging

Used for lightweight error logging in async loops:

```rust
dioxus_utils::console_log(
    format!("Error reading background data. Err:{:?}", err).as_str(),
);
```

`console_debug`, `console_info`, `console_warn` and `console_error` write with the matching
console level - see **Console Logging** below.

### Panic hook in `main()`

Every client app installs the panic hook first thing in `main()`. It is what shows a panic in the
browser console of a release (production) build - its text and the file, line and column it
happened at:

```rust
fn main() {
    dioxus_utils::set_panic_hook();
    dioxus::launch(App);
}
```

See **Panic Hook** below.

### JavaScript eval for UI helpers

Used in the toast helper to run small JS snippets:

```rust
let js = format!("document.getElementById('toast-message').innerText = \"{}\";", msg);
let _ = dioxus_utils::eval(js.as_str());
```

### UUID generation and date/time stamping

Both moved to `rust-extensions`, which handles wasm and native itself:

```rust
let id = if item.id.is_empty() {
    rust_extensions::uuid::generate_v4()
} else {
    item.id.clone()
};

let now = rust_extensions::date_time::DateTimeAsMicroseconds::now();
result.push_str(format!("Timestamp: {}", now.to_rfc3339()).as_str());
```

### Not used here (yet)

- `DialogValue` is not currently used in this repo.
- Focus helpers are implemented locally in `src/web/set_focus.rs`.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dioxus-utils = { tag = "{last_tag}", git = "https://github.com/MyJetTools/dioxus-utils.git" }
```

### Feature Flags

#### For Fullstack Applications

To enable fullstack features (client/server compatible code):

```toml
[dependencies]
dioxus-utils = { 
    tag = "{last_tag}", 
    git = "https://github.com/MyJetTools/dioxus-utils.git", 
    features = ["fullstack"] 
}

[features]
server = [..., "dioxus-utils/server"]
```

**Available Features:**
- `fullstack`: Enables fullstack utilities (focus, local storage, page reload, sleep)
- `server`: Enables server-side implementations (console logging, sleep, focus mock)
- `web`: Enables web-only utilities (`GlobalAppSettings`, `LOCAL_STORAGE` / `SESSION_STORAGE`, page reload, sleep, focus)

## Modules

### DataState

`DataState<T>` is a wrapper around `RenderState<T>` that tracks whether data has been loaded at least once. Useful for managing async data loading in components.

**States:**
- `None`: Initial state, no data loaded
- `Loading`: Data is currently being fetched
- `Loaded(T)`: Data successfully loaded
- `Error(String)`: Error occurred during loading

**Example:**

```rust
use dioxus::prelude::*;
use dioxus_utils::{DataState, RenderState};

fn MyComponent() -> Element {
    let mut data_state = use_signal(|| DataState::<Vec<String>>::new());

    match data_state.read().as_ref() {
        RenderState::None => {
            spawn(async move {
                data_state.write().set_loading();
                let result = fetch_data().await;
                match result {
                    Ok(data) => data_state.write().set_value(data),
                    Err(e) => data_state.write().set_error(e),
                }
            });
            rsx! { "Loading..." }
        }
        RenderState::Loading => rsx! { "Loading..." },
        RenderState::Loaded(data) => rsx! {
            for item in data {
                div { "{item}" }
            }
        },
        RenderState::Error(err) => rsx! { "Error: {err}" },
    }
}
```

**Key Methods:**
- `new()`: Create empty state
- `set_loading()`: Mark as loading
- `set_value(value)`: Set loaded data
- `set_error(err)`: Set error state
- `reset()`: Return to `None` to trigger a reload

### RenderState

`RenderState<T>` is the core state enum used by `DataState`. Can be used directly for simpler cases.

**Example:**

```rust
use dioxus_utils::RenderState;

let mut state = RenderState::<String>::new();
state.set_loading();
// ... later
state.set_loaded("Hello".to_string());
```

### DialogValue

`DialogValue<T>` tracks the initial and current value of a form field, useful for dialogs where you need to detect changes and allow cancellation.

**Example:**

```rust
use dioxus_utils::DialogValue;

fn EditDialog() -> Element {
    let mut name = use_signal(|| DialogValue::new("Initial Name".to_string()));
    
    rsx! {
        input {
            value: "{name.read().get_value()}",
            oninput: move |e| name.write().set_value(e.value()),
        }
        button {
            onclick: move |_| {
                if name.read().is_value_updated() {
                    // Save changes
                    save_name(name.read().get_value());
                }
            },
            "Save"
        }
        button {
            onclick: move |_| {
                // Reset to initial value
                name.write().init(name.read().get_init_value().clone());
            },
            "Cancel"
        }
    }
}
```

**Key Methods:**
- `new(value)`: Create with initial value
- `init(value)`: Reset both initial and current value
- `set_value(value)`: Update current value
- `get_value()`: Get current value
- `get_init_value()`: Get initial value
- `is_value_updated()`: Check if current differs from initial
- `get_value_mut()`: Get mutable reference to current value

### Console Logging

Platform-agnostic logging that works on both client and server. There is a function for each
console level:

| Function | Client (browser console) | Server (`server` feature) |
| --- | --- | --- |
| `console_log` | `console.log` | stdout |
| `console_debug` | `console.debug` | stdout |
| `console_info` | `console.info` | stdout |
| `console_warn` | `console.warn` | stderr |
| `console_error` | `console.error` | stderr |

Each of them takes the message as `&str`, `String` or `&String`.

**Example:**

```rust
use dioxus_utils::{console_error, console_info, console_log, console_warn};

console_log("Debug message");
console_log(format!("User ID: {}", user_id));
console_info("Connected");
console_warn(format!("Retrying in {} seconds", delay));
console_error(format!("Error reading background data. Err:{:?}", err));
```

Chrome DevTools shows `console.debug` messages only when the "Verbose" level is enabled.

### Panic Hook

`set_panic_hook()` writes every panic to the browser console as an error `panic: <text>`, with the
place of the panic on the next line:

```text
panic: called `Result::unwrap()` on an `Err` value: "boom"
    at src/main.rs:42:10
```

Call it once in `main()`, before the app is launched:

```rust
fn main() {
    dioxus_utils::set_panic_hook();
    dioxus::launch(App);
}
```

- **Release build**: this is what makes panics readable. Dioxus 0.7 installs no panic hook in a
  release build, so without `set_panic_hook()` a panic leaves only `RuntimeError: unreachable` in
  the browser console - no text and no place.
- **Debug build** (`dx serve`): Dioxus installs its own panic hook at launch, which replaces this
  one - the panic is printed in the Dioxus format, with a stack trace.
- **Server** (`server` feature): does nothing - a panic is already printed to stderr.
- **Call stack**: the line `at <file>:<line>:<column>` is the place of the panic itself. The stack
  trace which the browser attaches to the console entry has only numbered wasm frames
  (`wasm-function[2469]`) unless the app is built with `dx build --release --keep-names`, which
  keeps function names in the wasm binary (+24% of uncompressed wasm on a small test app).

### JavaScript Evaluation

`eval(js)` evaluates JavaScript code. On server, returns `JsValue::NULL`.

**Example:**

```rust
use dioxus_utils::eval;

let result = eval("Math.max(1, 2, 3)");
```

### UUID and Date/Time (moved to `rust-extensions`)

`generate_uuid()`, `now_date_time()` and `now_local_date_time()` have been removed - `rust-extensions`
provides both, with the wasm/native split handled inside it.

**Example:**

```rust
use rust_extensions::date_time::DateTimeAsMicroseconds;

let id = rust_extensions::uuid::generate_v4();
// Returns: "550e8400-e29b-41d4-a716-446655440000"

let now = DateTimeAsMicroseconds::now();
```

On non-wasm targets `rust_extensions::uuid::generate_v4()` requires the `rnd` feature of
`rust-extensions`.

### Local and Session Storage

Available when `web` feature is enabled (client-side apps).

`LOCAL_STORAGE` and `SESSION_STORAGE` are the objects to use for any access to the browser
`localStorage` / `sessionStorage`. Do not take the storage from `web_sys::window()` directly.

| Object | Browser storage | Lifetime of the data |
| --- | --- | --- |
| `dioxus_utils::js::LOCAL_STORAGE` | `window.localStorage` | Kept until deleted; shared by all tabs of the site |
| `dioxus_utils::js::SESSION_STORAGE` | `window.sessionStorage` | Per tab: survives a page refresh, gone when the tab is closed |

Both are `static` objects of type `WebStorage` with the same methods:

- `get(key) -> Option<String>`: `None` when there is no such key
- `set(key, value)`
- `delete(key)`

**Example:**

```rust
use dioxus_utils::js::{LOCAL_STORAGE, SESSION_STORAGE};

LOCAL_STORAGE.set("theme", "dark");
let theme = LOCAL_STORAGE.get("theme"); // Some("dark")

SESSION_STORAGE.set("draft", "some text");
SESSION_STORAGE.delete("draft");
```

`WebStorage` is `Copy`, so code which has to work with either storage takes it as a parameter:

```rust
use dioxus_utils::js::{WebStorage, SESSION_STORAGE};

fn load(storage: WebStorage, key: &str) -> String {
    storage.get(key).unwrap_or_default()
}

let draft = load(SESSION_STORAGE, "draft");
```

**How it works:**

- Nothing is requested from the browser until the first `get` / `set` / `delete`. That first call
  obtains the browser storage object once, and every later call reuses it.
- If the storage can not be obtained (no `window`, the browser returns `null`, or the browser denies
  access - blocked site data, a sandboxed iframe), `dioxus Local storage is not found: <reason>` or
  `dioxus Session storage is not found: <reason>` is written with `console_error`, and then the call
  panics with the same message. The console line is written separately because a release build
  prints no panic text unless the app has called `set_panic_hook()` - see **Panic Hook** above.
- `GlobalAppSettings::get_local_storage()` is kept for compatibility and returns `LOCAL_STORAGE`.

### Fullstack Utilities

Available when `fullstack` feature is enabled.

#### Set Focus

This repo uses a local helper (`src/web/set_focus.rs`) instead of the `dioxus-utils`
focus helper. If you want to switch to the library helper, add it and update usages.

#### Web Local Storage

`WebLocalStorage` provides access to browser local storage.

**Client**: Uses `web_sys::Storage`
**Server**: Mock implementation (no-op)

**Example:**

```rust
use dioxus_utils::js::fullstack::WebLocalStorage;

let storage = GlobalAppSettings::new().get_local_storage();
storage.set("key", "value");
let value = storage.get("key");
storage.delete("key");
```

#### Reload Page

`reload_page()` reloads the current page.

**Example:**

```rust
use dioxus_utils::js::fullstack::reload_page;

button {
    onclick: move |_| reload_page(),
    "Reload"
}
```

#### Sleep

`sleep(duration)` provides async sleep functionality.

**Client**: Uses `gloo-timers`
**Server**: Uses `tokio::time::sleep`

**Example:**

```rust
use dioxus_utils::js::fullstack::sleep;
use std::time::Duration;

async fn delayed_action() {
    sleep(Duration::from_secs(1)).await;
    // Continue after 1 second
}
```

### Global App Settings

`GlobalAppSettings` provides access to window location and local storage.

**Example:**

```rust
use dioxus_utils::js::GlobalAppSettings;

let href = GlobalAppSettings::get_href(); // Full URL
let origin = GlobalAppSettings::get_origin(); // Origin URL
let storage = GlobalAppSettings::get_local_storage();
```

In client-side (`web`) apps use `LOCAL_STORAGE` / `SESSION_STORAGE` for storage access - see
**Local and Session Storage** above.

## Complete Example

```rust
use dioxus::prelude::*;
use dioxus_utils::{
    DataState, RenderState, DialogValue, console_log,
};
use dioxus_utils::js::fullstack::*;

fn App() -> Element {
    let mut users = use_signal(|| DataState::<Vec<User>>::new());
    let mut edit_dialog = use_signal(|| None::<DialogValue<String>>);
    
    use_effect(move || {
        spawn(async move {
            users.write().set_loading();
            // Fetch users...
            users.write().set_loaded(vec![]);
        });
    });
    
    rsx! {
        match users.read().as_ref() {
            RenderState::Loading => rsx! { "Loading users..." },
            RenderState::Loaded(users_list) => rsx! {
                for user in users_list {
                    div { "{user.name}" }
                }
            },
            RenderState::Error(err) => rsx! { "Error: {err}" },
            _ => rsx! {},
        }
        
        button {
            onclick: move |_| {
                let id = rust_extensions::uuid::generate_v4();
                console_log(&format!("Created user with ID: {}", id));
            },
            "Create User"
        }
    }
}
```

## Dependencies

- `dioxus`: Core Dioxus framework
- `web-sys`: WebAssembly bindings for web APIs
- `js-sys`: JavaScript bindings
- `rust-extensions`: Utility extensions (`StrOrString`, uuid and date/time helpers)
- `gloo-timers`: Timer utilities for web
- `tokio`: Async runtime (optional, for server feature)

## Platform Support

- **Web**: Full support for all features
- **Server**: Supported features when `server` feature is enabled
- **Fullstack**: Seamless client/server code sharing with `fullstack` feature

## License

[Add your license here]

## Contributing

[Add contribution guidelines here]

## Repository

https://github.com/MyJetTools/dioxus-utils

## `NotifyChildComponent<TValue>`

A utility for delivering update events from a parent component to child components that manage their own state.

The parent holds an instance (created via `new()`) and calls `notify_other_components(value)` after a mutation. Child components call `on_notify(callback)` as a hook — it subscribes via `use_effect` and fires the callback when a notification arrives. The child then resets its `DataState` to trigger a reload.

Because `NotifyChildComponent<TValue>` wraps a `Signal` internally, it is `Copy + Clone` and can be passed directly as a component prop.
