#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum LookupResult {
    AlwaysInstalled,
    NeverInstalled,
    InstallWith(String),
}
