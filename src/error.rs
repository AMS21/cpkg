#[derive(thiserror::Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    #[error("Generic error: '{0}'")]
    Generic(String),

    // TODO: Remove me and replace with specialized error types
    #[error("Static error: '{0}'")]
    Static(&'static str),
}
