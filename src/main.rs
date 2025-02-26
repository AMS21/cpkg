use std::process::ExitCode;

use cpkg::prelude::*;
use cpkg::run_main;

const EXIT_FAILURE: u8 = 1;
const EXIT_CRASH: u8 = 255;

fn handle_error(error: Error) -> u8 {
    match error {
        // -- User errors --
        Error::CommandFailed {
            exit_code,
            command_line,
        } => {
            eprintln!("Subcommand failed with {exit_code}");
            eprintln!("Full commandline: {command_line}");

            EXIT_FAILURE
        }

        Error::PackageNotFound { package_name } => {
            eprintln!("Package not found: '{package_name}'");

            EXIT_FAILURE
        }

        Error::OptionNotSupported {
            option_name,
            operation,
            provider,
        } => {
            eprintln!("Option '{option_name}' is not supported for '{operation}' by {provider}");

            EXIT_FAILURE
        }

        Error::NoSuperuserLauncherFound { provider } => {
            eprintln!(
                "Provider '{provider}' requires superuser privileges but no suitable launcher was found"
            );

            EXIT_FAILURE
        }

        // -- Internal errors --
        #[expect(clippy::use_debug)]
        Error::OsStringConversion { .. }
        | Error::ClapArguments { .. }
        | Error::IO { .. }
        | Error::TOMLDeserialization { .. }
        | Error::TOMLSerialization { .. } => {
            eprintln!("Oh no. Internal error: {error:?}");
            eprintln!("Please report this issue at https://github.com/AMS21/cpkg/issues");

            EXIT_CRASH
        }
    }
}

fn main() -> ExitCode {
    let result = run_main();

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => ExitCode::from(handle_error(error)),
    }
}
