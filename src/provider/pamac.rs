use crate::prelude::*;
use crate::provider::Provider;
use crate::subcommand::install;
use std::path::PathBuf;
use which::which;

// TODO: Instead of installed boolean just have executable_path as Option
// TODO: Generalize a lot of this maybe this a macro?

#[allow(clippy::module_name_repetitions)]
pub struct PamacProvider {
    executable_path: PathBuf,
    installed: bool,
}

impl Provider for PamacProvider {
    fn name(&self) -> &'static str {
        "pamac"
    }

    fn initialize() -> Self {
        which("pamac").map_or_else(
            |_| Self {
                executable_path: PathBuf::new(),
                installed: false,
            },
            |apt_path| Self {
                executable_path: apt_path,
                installed: true,
            },
        )
    }

    fn is_installed(&self) -> bool {
        self.installed
    }

    fn lookup_package(
        &self,
        database: &crate::database::Database,
        package_name: &str,
    ) -> Option<String> {
        if let Some(application) = database.packages.get(package_name) {
            if let Some(pamac_string) = &application.pamac {
                return Some(pamac_string.to_owned());
            }

            return Some(package_name.to_owned());
        }

        None
    }

    fn install_packages(
        &self,
        database: &crate::database::Database,
        packages: &[&String],
        options: &install::Options,
    ) -> Result<()> {
        let mut command = std::process::Command::new(&self.executable_path);

        command.arg("install");

        // TODO: This code is duplicated with apt.rs move it outside and make it generic
        // Now add all the translated package names
        for package in packages {
            if let Some(apt_package_name) = self.lookup_package(database, package) {
                command.arg(apt_package_name);
            } else {
                // TODO: Don't return generic error
                return Err(Error::Generic(format!(
                    "Package '{package}' not found in database"
                )));
            }
        }

        // Handle assume yes
        if options.assume_yes {
            command.arg("--no-confirm");
        }

        // Handle dry run
        if options.dry_run {
            command.arg("-d");
        }

        // run the actual command
        command.spawn()?.wait()?;

        Ok(())
    }

    fn remove_packages(
        &self,
        database: &crate::database::Database,
        packages: &[&String],
        options: &crate::subcommand::remove::Options,
    ) -> Result<()> {
        let mut command = std::process::Command::new(&self.executable_path);

        command.arg("remove");

        // TODO: This code is duplicated with apt.rs move it outside and make it generic
        // Now add all the translated package names
        for package in packages {
            if let Some(apt_package_name) = self.lookup_package(database, package) {
                command.arg(apt_package_name);
            } else {
                // TODO: Don't return generic error
                return Err(Error::Generic(format!(
                    "Package '{package}' not found in database"
                )));
            }
        }

        // Handle assume yes
        if options.assume_yes {
            command.arg("--no-confirm");
        }

        // Handle dry run
        if options.dry_run {
            command.arg("-d");
        }

        // run the actual command
        command.spawn()?.wait()?;

        Ok(())
    }
}
