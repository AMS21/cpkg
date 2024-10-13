use cpkg::prelude::*;
use cpkg::run_main;

const EXIT_FAILURE: i32 = 1;
const EXIT_CRASH: i32 = -1;

fn handle_error(error: Error) -> i32 {
    match error {
        // -- User errors --
        Error::InstallCommandFailed {
            exit_code,
            command_line,
        } => {
            eprintln!("Package installation failed with {exit_code}");
            eprintln!("Full commandline: {command_line}");

            EXIT_FAILURE
        }
        Error::RemoveCommandFailed {
            exit_code,
            command_line,
        } => {
            eprintln!("Package removal failed with {exit_code}");
            eprintln!("Full commandline: {command_line}");

            EXIT_FAILURE
        }
        Error::ReinstallCommandFailed {
            exit_code,
            command_line,
        } => {
            eprintln!("Package reinstall failed with {exit_code}");
            eprintln!("Full commandline: {command_line}");

            EXIT_FAILURE
        }
        Error::PackageNotFound { package_name } => {
            eprintln!("Package not found: '{package_name}'");

            EXIT_FAILURE
        }

        // -- Internal errors --
        #[allow(clippy::use_debug)]
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

fn main() {
    let result = run_main();

    match result {
        Ok(()) => {}

        Err(error) => {
            let exit_code = handle_error(error);

            #[allow(clippy::exit)]
            std::process::exit(exit_code);
        }
    }
}
