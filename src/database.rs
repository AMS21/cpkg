use crate::application::Application;
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Database {
    #[serde(flatten)]
    pub packages: HashMap<String, Application>,
}

pub fn load_from_file(path: &str) -> Result<Database> {
    let database_string = read_to_string(path)?;

    // Deserialize
    let database = toml::from_str(&database_string)?;

    Ok(database)
}
