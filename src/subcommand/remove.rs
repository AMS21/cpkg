use crate::database;
use crate::prelude::*;
use crate::provider;
use clap::Arg;
use clap::ArgAction;
use clap::Command;

pub const SUBCOMMAND_NAME: &str = "remove";

const ARG_PACKAGES: &str = "PACKAGES";
const ARG_ASSUME_YES: &str = "YES";
const ARG_DRY_RUN: &str = "DRY_RUN";

#[must_use]
pub fn get_subcommand() -> Command {
    Command::new(SUBCOMMAND_NAME)
        .about("Removes given package(s)")
        .args([
            Arg::new(ARG_PACKAGES).num_args(1..).required(true),
            Arg::new(ARG_ASSUME_YES)
                .short('y')
                .long("assume-yes")
                .alias("yes")
                .action(ArgAction::SetTrue)
                .help("Assume yes for all confirmation prompts"),
            Arg::new(ARG_DRY_RUN)
                .short('d')
                .long("dry-run")
                .alias("simulate")
                .action(ArgAction::SetTrue)
                .help("Only print what would be done rather than actually doing it"),
        ])
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Options {
    pub assume_yes: bool,
    pub dry_run: bool,
}

#[allow(clippy::use_debug)]
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    // Get all packages the user wants to remove from the command line
    let packages: Vec<&String> = matches
        .get_many::<String>(ARG_PACKAGES)
        .ok_or_else(|| Error::ClapArguments("PACKAGES argument should have been set"))?
        .collect();

    // Get options
    let options = Options {
        assume_yes: matches.get_flag(ARG_ASSUME_YES),
        dry_run: matches.get_flag(ARG_DRY_RUN),
    };

    // Load database
    // TODO: Don't hardcode database path
    let database = database::load_from_file("database.toml")?;

    let providers = provider::get_all_providers();

    // TODO: Use a hiarchey to remove and not all of them
    for provider in providers {
        if !provider.is_installed() {
            continue;
        }

        // Translate packages
        let mut translated_packages = Vec::with_capacity(packages.len());

        for package_name in &packages {
            if let Some(application) = database.packages.get(package_name as &str) {
                translated_packages.push(provider.lookup_package(application, package_name));
            } else {
                // TODO: Don't return generic error
                return Err(Error::Generic(format!(
                    "Package '{package_name}' not found in database"
                )));
            }
        }

        println!("Removing {:?} with {}", packages, provider.name());
        provider.remove_packages(&translated_packages, &options)?;
    }

    Ok(())
}
