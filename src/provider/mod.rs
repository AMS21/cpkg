use crate::application::Application;
use crate::prelude::*;
use crate::subcommand::install;
use crate::subcommand::remove;

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

    fn lookup_package(&self, application: &Application, package_name: &str) -> String;

    fn install_packages(&self, packages: &[String], options: &install::Options) -> Result<()>;

    fn remove_packages(&self, packages: &[String], options: &remove::Options) -> Result<()>;
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

#[must_use]
pub fn get_number_of_installed_providers() -> usize {
    get_all_providers()
        .iter()
        .filter(|provider| provider.is_installed())
        .count()
}
