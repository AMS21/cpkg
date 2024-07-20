use crate::database;
use crate::prelude::*;
use crate::provider;
use clap::Arg;
use clap::Command;

pub const SUBCOMMAND_NAME: &str = "install";

pub const ARG_PACKAGES: &str = "PACKAGES";
pub const ARG_ASSUME_YES: &str = "YES";

#[must_use]
pub fn get_subcommand() -> clap::Command {
    Command::new(SUBCOMMAND_NAME)
        .about("Installs given package(s)")
        .arg(Arg::new(ARG_PACKAGES).num_args(1..).required(true))
        .arg(
            Arg::new(ARG_ASSUME_YES)
                .short('y')
                .long("assume-yes")
                .action(clap::ArgAction::SetTrue)
                .help("Assume yes for all confirmation prompts"),
        )
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Options {
    pub assume_yes: bool,
}

pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    // Get all packages the user wants to install
    let packages: Vec<&String> = matches.get_many::<String>(ARG_PACKAGES).unwrap().collect();

    // Get options
    let options = Options {
        assume_yes: matches.get_flag(ARG_ASSUME_YES),
    };

    // Load database
    // TODO: Don't hardcode database path
    let database = database::load_from_file("database.toml")?;

    let providers = provider::get_all_providers();

    // TODO: Use a hiarchey to install and not all of them
    for provider in providers {
        if provider.is_installed() {
            println!("Installing {:?} with {}", packages, provider.name());
            provider.install_packages(&database, packages.as_slice(), &options)?;
        }
    }

    Ok(())
}
