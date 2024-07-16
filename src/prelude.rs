pub use crate::error::Error;

pub type Result<T> = color_eyre::Result<T, Error>;
