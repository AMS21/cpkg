use crate::cli::ARGUMENT_ASSUME_YES;
use crate::cli::ARGUMENT_DRY_RUN;
use crate::cli::ARGUMENT_PACKAGES;
use crate::database;
use crate::prelude::*;
use crate::provider;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Options {
    pub assume_yes: bool,
    pub dry_run: bool,
}

#[allow(clippy::use_debug)]
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    // Get all packages the user wants to install from the command line
    let packages: Vec<&String> = matches
        .get_many::<String>(ARGUMENT_PACKAGES)
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

    let providers = provider::get_installed_providers();

    // TODO: Use a hiarchey to install and not all of them
    for provider in providers {
        debug_assert!(provider.is_installed(), "Provider should be installed");

        // Translate packages
        let mut translated_packages = Vec::with_capacity(packages.len());

        for package_name in &packages {
            if let Some(application) = database.packages.get(package_name as &str) {
                translated_packages.push(provider.lookup_package(application, package_name));
            } else {
                return Err(Error::PackageNotFound {
                    package_name: (*package_name).to_string(),
                });
            }
        }

        println!("Installing {:?} with {}", packages, provider.name());
        provider.install_packages(&translated_packages, &options)?;
    }

    Ok(())
}
