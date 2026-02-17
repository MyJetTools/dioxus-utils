use std::{collections::BTreeMap, fmt::Display};

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
    pub fn build(&self) -> Vec<u8> {
        serde_json::to_vec(&self.fields).unwrap()
    }
}
