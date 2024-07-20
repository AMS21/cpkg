use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, Default)]
pub struct Application {
    pub apt: Option<String>,
    pub pamac: Option<String>,
}
