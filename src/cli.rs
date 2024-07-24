use clap::crate_authors;
use clap::crate_description;
use clap::crate_name;
use clap::crate_version;
use clap::Command;

use crate::subcommand;

#[must_use]
pub fn get_command_line() -> Command {
    Command::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .arg_required_else_help(true)
        .subcommands([
            subcommand::install::get_subcommand(),
            subcommand::remove::get_subcommand(),
        ])
}
