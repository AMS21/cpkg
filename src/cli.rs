use clap::crate_authors;
use clap::crate_description;
use clap::crate_name;
use clap::crate_version;
use clap::Arg;
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

pub const ARGUMENT_ASSUME_YES: &str = "YES";

#[must_use]
pub fn get_argument_assume_yes() -> Arg {
    Arg::new(ARGUMENT_ASSUME_YES)
        .short('y')
        .long("assume-yes")
        .alias("yes")
        .action(clap::ArgAction::SetTrue)
        .help("Assume yes for all confirmation prompts")
}

pub const ARGUMENT_DRY_RUN: &str = "DRY_RUN";

#[must_use]
pub fn get_argument_dry_run() -> Arg {
    Arg::new(ARGUMENT_DRY_RUN)
        .short('d')
        .long("dry-run")
        .alias("simulate")
        .action(clap::ArgAction::SetTrue)
        .help("Only print what would be done rather than actually doing it")
}

pub const ARGUMENT_PACKAGES: &str = "PACKAGES";

#[must_use]
pub fn get_argument_packages_list() -> Arg {
    Arg::new(ARGUMENT_PACKAGES).num_args(1..).required(true)
}
