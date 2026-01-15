# dioxus-utils

A comprehensive utility library for Dioxus applications providing state management, browser APIs, and fullstack development utilities.

## Overview

`dioxus-utils` is a collection of utilities designed to simplify common tasks in Dioxus applications. It provides abstractions for data loading states, dialog/form management, browser interactions, and seamless client/server code sharing for fullstack applications.

## Features

- **State Management**: `DataState` and `RenderState` for managing async data loading states
- **Dialog Management**: `DialogValue` for tracking form changes in dialogs
- **Browser Utilities**: Console logging, JavaScript evaluation, UUID generation, date/time handling
- **Fullstack Support**: Client/server compatible utilities for focus management, local storage, page reload, and async sleep
- **Global Settings**: Access to window location and local storage through `GlobalAppSettings`

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

### Local storage via `GlobalAppSettings`

Used for lightweight client-side persistence:

```rust
use dioxus_utils::js::GlobalAppSettings;

const STORAGE_KEY: &str = "client-view-search";

pub fn get() -> Vec<String> {
    let result = GlobalAppSettings::get_local_storage().get(STORAGE_KEY);
    result
        .unwrap_or_default()
        .split(';')
        .map(|itm| itm.to_string())
        .collect()
}

pub fn save(items: &[String]) {
    let joined = items.join(";");
    GlobalAppSettings::get_local_storage().set(STORAGE_KEY, joined.as_str());
}
```

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

### JavaScript eval for UI helpers

Used in the toast helper to run small JS snippets:

```rust
let js = format!("document.getElementById('toast-message').innerText = \"{}\";", msg);
let _ = dioxus_utils::eval(js.as_str());
```

### UUID generation

Used when creating new items with empty ids:

```rust
let id = if item.id.is_empty() {
    dioxus_utils::generate_uuid()
} else {
    item.id.clone()
};
```

### Date/time stamping

Used when building export payloads:

```rust
let now = dioxus_utils::now_date_time();
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
- `server`: Enables server-side implementations (UUID generation, date/time, console logging)

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

`console_log()` provides platform-agnostic logging that works on both client and server.

**Client**: Logs to browser console via JavaScript
**Server**: Logs to stdout

**Example:**

```rust
use dioxus_utils::console_log;

console_log("Debug message");
console_log(format!("User ID: {}", user_id));
```

### JavaScript Evaluation

`eval(js)` evaluates JavaScript code. On server, returns `JsValue::NULL`.

**Example:**

```rust
use dioxus_utils::eval;

let result = eval("Math.max(1, 2, 3)");
```

### UUID Generation

`generate_uuid()` generates a UUID v4 string.

**Client**: Uses `crypto.randomUUID()` via JavaScript
**Server**: Uses `uuid` crate

**Example:**

```rust
use dioxus_utils::generate_uuid;

let id = generate_uuid();
// Returns: "550e8400-e29b-41d4-a716-446655440000"
```

### Date/Time Utilities

`now_date_time()` returns current date/time as `DateTimeAsMicroseconds` from `rust-extensions`.

**Client**: Uses JavaScript `Date().toISOString()`
**Server**: Uses system time

**Example:**

```rust
use dioxus_utils::now_date_time;

let now = now_date_time();
```

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

## Complete Example

```rust
use dioxus::prelude::*;
use dioxus_utils::{
    DataState, RenderState, DialogValue,
    console_log, generate_uuid, now_date_time,
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
                let id = generate_uuid();
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
- `rust-extensions`: Utility extensions (for DateTimeAsMicroseconds)
- `gloo-timers`: Timer utilities for web
- `tokio`: Async runtime (optional, for server feature)
- `uuid`: UUID generation (optional, for server feature)

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
