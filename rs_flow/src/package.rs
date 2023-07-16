use serde::Serialize;
use serde_json::{self, Value, };

#[derive(Clone)]
pub struct Package {
    content: Value
}

impl Package {
    pub fn new<T: Serialize>(content: T) -> Self {
        let value = serde_json::to_value(content).unwrap_or(Value::Null);
        Self { content: value }
    }

    pub fn content(&self) -> &Value {
        &self.content
    }
}