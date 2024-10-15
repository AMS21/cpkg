pub mod application;
pub mod cli;
pub mod database;
pub mod error;
pub mod prelude;
pub mod provider;
pub mod subcommand;
pub mod utility;

use prelude::*;
use subcommand::install;
use subcommand::reinstall;
use subcommand::remove;
use subcommand::update;

pub fn run_main() -> Result<()> {
    // Handle command line arguments
    let matches = cli::get_command_line().get_matches();

    match matches.subcommand() {
        Some((cli::SUBCOMMAND_INSTALL, sub_matches)) => {
            install::run(sub_matches)?;
        }

        Some((cli::SUBCOMMAND_REMOVE, sub_matches)) => {
            remove::run(sub_matches)?;
        }

        Some((cli::SUBCOMMAND_REINSTALL, sub_matches)) => {
            reinstall::run(sub_matches)?;
        }

        Some((cli::SUBCOMMAND_UPDATE, sub_matches)) => {
            update::run(sub_matches)?;
        }

        #[allow(clippy::unreachable)]
        _ => unreachable!("All subcommands should be defined"),
    }

    Ok(())
}
