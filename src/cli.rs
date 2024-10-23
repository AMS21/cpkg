use clap::crate_authors;
use clap::crate_description;
use clap::crate_version;
use clap::Arg;
use clap::Command;

pub const COMMAND_NAME: &str = "cpkg";

#[must_use]
pub fn get_command_line() -> Command {
    Command::new(COMMAND_NAME)
        .about(crate_description!())
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .propagate_version(true)
        .arg_required_else_help(true)
        .subcommands([
            get_install_subcommand(),
            get_remove_subcommand(),
            get_reinstall_subcommand(),
            get_update_subcommand(),
        ])
}

// Commands

pub const SUBCOMMAND_INSTALL: &str = "install";

#[must_use]
fn get_install_subcommand() -> Command {
    Command::new(SUBCOMMAND_INSTALL)
        .about("Installs given package(s)")
        .visible_aliases(["add"])
        .args([
            get_argument_packages_list(),
            get_argument_assume_yes(),
            get_argument_dry_run(),
        ])
}

pub const SUBCOMMAND_REMOVE: &str = "remove";

#[must_use]
pub fn get_remove_subcommand() -> Command {
    Command::new(SUBCOMMAND_REMOVE)
        .about("Removes given package(s)")
        .visible_aliases(["uninstall", "delete"])
        .args([
            get_argument_packages_list(),
            get_argument_assume_yes(),
            get_argument_dry_run(),
        ])
}

pub const SUBCOMMAND_REINSTALL: &str = "reinstall";

#[must_use]
pub fn get_reinstall_subcommand() -> Command {
    Command::new(SUBCOMMAND_REINSTALL)
        .about("Reinstalls given package(s)")
        .args([
            get_argument_packages_list(),
            get_argument_assume_yes(),
            get_argument_dry_run(),
        ])
}

pub const SUBCOMMAND_UPDATE: &str = "update";

#[must_use]
pub fn get_update_subcommand() -> Command {
    Command::new(SUBCOMMAND_UPDATE)
        .about("Updates all installed packages")
        .visible_alias("upgrade")
        .args([get_argument_assume_yes(), get_argument_dry_run()])
}

// Arguments

pub const ARGUMENT_ASSUME_YES: &str = "YES";

#[must_use]
pub fn get_argument_assume_yes() -> Arg {
    Arg::new(ARGUMENT_ASSUME_YES)
        .short('y')
        .long("assume-yes")
        .visible_alias("yes")
        .action(clap::ArgAction::SetTrue)
        .help("Assume yes for all confirmation prompts")
}

pub const ARGUMENT_DRY_RUN: &str = "DRY_RUN";

#[must_use]
pub fn get_argument_dry_run() -> Arg {
    Arg::new(ARGUMENT_DRY_RUN)
        .short('d')
        .long("dry-run")
        .visible_alias("simulate")
        .action(clap::ArgAction::SetTrue)
        .help("Only print what would be done rather than actually doing it")
}

pub const ARGUMENT_PACKAGES: &str = "PACKAGES";

#[must_use]
pub fn get_argument_packages_list() -> Arg {
    Arg::new(ARGUMENT_PACKAGES).num_args(1..).required(true)
}
