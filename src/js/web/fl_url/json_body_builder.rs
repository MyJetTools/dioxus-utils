use std::{collections::BTreeMap, fmt::Display};

use super::HttpRequestBody;

pub struct JsonBodyBuilder {
    fields: BTreeMap<String, serde_json::Value>,
}

impl JsonBodyBuilder {
    pub fn new() -> Self {
        Self {
            fields: Default::default(),
        }
    }

    pub fn with_field(mut self, name: impl Display, value: impl Into<serde_json::Value>) -> Self {
        let name = format!("{}", name);
        self.fields.insert(name, value.into());

        self
    }
    pub fn build(&self) -> HttpRequestBody {
        HttpRequestBody::as_json(&self.fields)
    }
}
