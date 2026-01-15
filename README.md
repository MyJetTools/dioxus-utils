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
use dioxus_utils::DataState;

fn MyComponent() -> Element {
    let mut data_state = use_signal(|| DataState::<Vec<String>>::new());
    
    use_effect(move || {
        spawn(async move {
            data_state.write().set_loading();
            // Simulate async data fetch
            let result = fetch_data().await;
            match result {
                Ok(data) => data_state.write().set_loaded(data),
                Err(e) => data_state.write().set_error(e),
            }
        });
    });
    
    rsx! {
        match data_state.read().as_ref() {
            RenderState::None => rsx! { "No data" },
            RenderState::Loading => rsx! { "Loading..." },
            RenderState::Loaded(data) => rsx! {
                for item in data {
                    div { "{item}" }
                }
            },
            RenderState::Error(err) => rsx! { "Error: {err}" },
        }
    }
}
```

**Key Methods:**
- `new()`: Create empty state
- `new_as_loaded(value)`: Create with initial loaded value
- `set_loading()`: Mark as loading
- `set_loaded(value)`: Set loaded data
- `set_error(err)`: Set error state
- `is_loading()`: Check if loading
- `has_value()`: Check if has loaded value
- `try_unwrap_as_loaded()`: Get reference to loaded value (Option)
- `unwrap_as_loaded()`: Get reference to loaded value (panics if not loaded)
- `had_data_loaded_once`: Boolean flag indicating if data was ever loaded

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

`set_focus(id, set_focus_signal)` focuses an element by ID. Uses a signal to prevent multiple focus attempts.

**Client**: Focuses DOM element
**Server**: No-op

**Example:**

```rust
use dioxus_utils::js::fullstack::set_focus;
use dioxus::prelude::*;

fn MyComponent() -> Element {
    let mut should_focus = use_signal(|| false);
    
    rsx! {
        input {
            id: "my-input",
            // ...
        }
        button {
            onclick: move |_| {
                set_focus("my-input", should_focus);
            },
            "Focus Input"
        }
    }
}
```

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
use dioxus_utils::js::fullstack::GlobalAppSettings;

let settings = GlobalAppSettings::new();
let href = settings.get_href(); // Full URL
let origin = settings.get_origin(); // Origin URL
let storage = settings.get_local_storage();
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
