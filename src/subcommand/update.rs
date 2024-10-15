use crate::cli::ARGUMENT_ASSUME_YES;
use crate::cli::ARGUMENT_DRY_RUN;
use crate::prelude::*;
use crate::provider;

// TODO: Add support for updating specific packages

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Options {
    pub assume_yes: bool,
    pub dry_run: bool,
}

#[allow(clippy::use_debug)]
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    // Get options
    let options = Options {
        assume_yes: matches.get_flag(ARGUMENT_ASSUME_YES),
        dry_run: matches.get_flag(ARGUMENT_DRY_RUN),
    };

    // Get all providers
    let providers = provider::get_installed_providers();

    for provider in providers {
        debug_assert!(provider.is_installed(), "Provider should be installed");

        println!("Updating package manager {}", provider.name());
        provider.update_packages(&options)?;
    }

    Ok(())
}
