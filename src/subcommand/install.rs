use crate::cli::get_argument_assume_yes;
use crate::cli::get_argument_dry_run;
use crate::cli::get_argument_packages_list;
use crate::cli::ARGUMENT_ASSUME_YES;
use crate::cli::ARGUMENT_DRY_RUN;
use crate::database;
use crate::prelude::*;
use crate::provider;
use clap::Command;

pub const SUBCOMMAND_NAME: &str = "install";

const ARG_PACKAGES: &str = "PACKAGES";

#[must_use]
pub fn get_subcommand() -> clap::Command {
    Command::new(SUBCOMMAND_NAME)
        .about("Installs given package(s)")
        .args([
            get_argument_packages_list(),
            get_argument_assume_yes(),
            get_argument_dry_run(),
        ])
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Options {
    pub assume_yes: bool,
    pub dry_run: bool,
}

#[allow(clippy::use_debug)]
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    // Get all packages the user wants to install from the command line
    let packages: Vec<&String> = matches
        .get_many::<String>(ARG_PACKAGES)
        .ok_or_else(|| Error::ClapArguments("PACKAGES argument should have been set"))?
        .collect();

    // Get options
    let options = Options {
        assume_yes: matches.get_flag(ARGUMENT_ASSUME_YES),
        dry_run: matches.get_flag(ARGUMENT_DRY_RUN),
    };

    // Load database
    // TODO: Don't hardcode database path
    let database = database::load_from_file("database.toml")?;

    let providers = provider::get_all_providers();

    // TODO: Use a hiarchey to install and not all of them
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

        println!("Installing {:?} with {}", packages, provider.name());
        provider.install_packages(&translated_packages, &options)?;
    }

    Ok(())
}
