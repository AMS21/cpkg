pub mod application;
pub mod cli;
pub mod database;
pub mod error;
pub mod prelude;
pub mod provider;
pub mod subcommand;
pub mod utility;

use crate::prelude::*;
use subcommand::install;
use subcommand::remove;

fn main() -> Result<()> {
    color_eyre::install()?;

    // Handle command line arguments
    let command_line = cli::get_command_line();
    let matches = command_line.get_matches();

    match matches.subcommand() {
        Some((install::SUBCOMMAND_NAME, sub_matches)) => {
            install::run(sub_matches)?;
        }

        Some((remove::SUBCOMMAND_NAME, sub_matches)) => {
            remove::run(sub_matches)?;
        }

        #[allow(clippy::unreachable)]
        _ => unreachable!("All subcommands should be defined"),
    }

    Ok(())
}
