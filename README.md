### For fullstack application please add to the cargo.toml


```toml
[features]
server = [.., "dioxus-utils/server"]

dioxus-utils = { tag = "x.x.x", git = "https://github.com/MyJetTools/dioxus-utils.git", features = [
    "fullstack",
] }

```