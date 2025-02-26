use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Bool(bool),
    String(String),
    Object(HashMap<String, Value>),
}

impl Default for Value {
    fn default() -> Self {
        Self::String(String::default())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
pub struct Application {
    pub apk: Option<Value>,
    pub apt: Option<Value>,
    pub dnf: Option<Value>,
    pub flatpak: Option<Value>,
    pub pamac: Option<Value>,
}
