use crate::database::Database;
use crate::prelude::*;
use crate::provider::Provider;
use crate::subcommand::install;
use crate::utility::file_exists;

#[allow(clippy::module_name_repetitions)]
pub struct AptProvider {
    executable_path: String,
    installed: bool,
}

const USER_APT_PATH: &str = "/usr/bin/apt";
const SYSTEM_APT_PATH: &str = "/bin/apt";

impl Provider for AptProvider {
    fn name(&self) -> &'static str {
        "apt"
    }

    fn initialize() -> Self {
        if file_exists(USER_APT_PATH) {
            return Self {
                executable_path: USER_APT_PATH.to_owned(),
                installed: true,
            };
        } else if file_exists(SYSTEM_APT_PATH) {
            return Self {
                executable_path: SYSTEM_APT_PATH.to_owned(),
                installed: true,
            };
        }

        Self {
            executable_path: String::new(),
            installed: false,
        }
    }

    fn is_installed(&self) -> bool {
        self.installed
    }

    fn lookup_package(&self, database: &Database, package_name: &str) -> Option<String> {
        if let Some(app) = database.packages.get(package_name) {
            if let Some(apt_string) = &app.apt {
                return Some(apt_string.to_string());
            }

            return Some(package_name.to_owned());
        }

        None
    }

    fn install_packages(
        &self,
        database: &Database,
        packages: &[&String],
        options: &install::Options,
    ) -> Result<()> {
        let mut command = std::process::Command::new(&self.executable_path);

        command.arg("install");

        // Now add all the translated package names
        for package in packages {
            if let Some(apt_package_name) = self.lookup_package(database, package) {
                command.arg(apt_package_name);
            } else {
                return Err(Error::Generic(format!(
                    "Package '{package}' not found in database"
                )));
            }
        }

        // Don't install recommended packages
        command.arg("--no-install-recommends");

        // Add -y if assume_yes is true
        if options.assume_yes {
            command.arg("-y");
        }

        // run the actual command
        command.spawn()?.wait()?;

        Ok(())
    }
}
