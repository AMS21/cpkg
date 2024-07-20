#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error: '{0}'")]
    Generic(String),

    // TODO: Remove me and replace with specialized error types
    #[error("Static error: '{0}'")]
    Static(&'static str),

    #[error("Failed to install color_eyre")]
    ColorEyreInstall(#[from] color_eyre::Report),

    #[error("IO error")]
    IO(#[from] std::io::Error),

    #[error("TOML deserialization error")]
    TOMLDeserialization(#[from] toml::de::Error),

    #[error("TOML serialization error")]
    TOMLSerialization(#[from] toml::ser::Error),
}
