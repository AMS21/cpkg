use crate::database::Database;
use crate::prelude::*;
use crate::subcommand::install;

#[cfg(feature = "apt")]
pub mod apt;
#[cfg(feature = "pamac")]
pub mod pamac;

pub trait Provider {
    #[must_use]
    fn name(&self) -> &'static str;

    #[must_use]
    fn initialize() -> Self
    where
        Self: Sized;

    #[must_use]
    fn is_installed(&self) -> bool;

    fn lookup_package(&self, database: &Database, package_name: &str) -> Option<String>;

    fn install_packages(
        &self,
        database: &Database,
        packages: &[&String],
        options: &install::Options,
    ) -> Result<()>;
}

#[must_use]
pub fn get_all_providers() -> Vec<Box<dyn Provider>> {
    vec![
        #[cfg(feature = "apt")]
        Box::new(apt::AptProvider::initialize()),
        #[cfg(feature = "pamac")]
        Box::new(pamac::PamacProvider::initialize()),
    ]
}
