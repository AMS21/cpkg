use derive_more::From;
use std::ffi::OsString;
use std::process::ExitStatus;

#[derive(Debug, From)]
pub enum Error {
    // -- User errors --
    CommandFailed {
        exit_code: ExitStatus,
        command_line: String,
    },

    PackageNotFound {
        package_name: String,
    },

    OptionNotSupported {
        option_name: &'static str,
        operation: &'static str,
        provider: &'static str,
    },

    // -- Internal errors --
    OsStringConversion {
        original: OsString,
    },

    // -- Externals --
    #[from]
    IO(std::io::Error),

    #[from]
    TOMLDeserialization(toml::de::Error),

    #[from]
    TOMLSerialization(toml::ser::Error),

    #[from]
    ClapArguments(&'static str),
}

#[allow(clippy::use_debug)]
impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
