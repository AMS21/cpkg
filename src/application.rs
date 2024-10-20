use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, Default)]
pub struct Application {
    pub apk: Option<String>,
    pub apt: Option<String>,
    pub dnf: Option<String>,
    pub pamac: Option<String>,
}
