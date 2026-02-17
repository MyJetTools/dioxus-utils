use std::{collections::BTreeMap, fmt::Display};

use super::super::HttpRequestBody;

pub struct JsonBodyBuilder {
    fields: BTreeMap<String, String>,
}

impl JsonBodyBuilder {
    pub fn new() -> Self {
        Self {
            fields: Default::default(),
        }
    }

    pub fn with_field(mut self, name: impl Display, value: impl Display) -> Self {
        let name = format!("{}", name);
        let value = format!("{}", value);

        self.fields.insert(name, value);

        self
    }
    pub fn build(&self) -> HttpRequestBody {
        HttpRequestBody::as_json(&self.fields)
    }
}
