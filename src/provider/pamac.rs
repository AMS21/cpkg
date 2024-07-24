use crate::application::Application;
use crate::prelude::*;
use crate::provider::Provider;
use crate::subcommand::install;
use crate::subcommand::remove;
use std::path::PathBuf;
use which::which;

// TODO: Instead of installed boolean just have executable_path as Option
// TODO: Generalize a lot of this maybe with a macro?

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

    fn lookup_package(&self, application: &Application, package_name: &str) -> String {
        if let Some(pamac_string) = &application.pamac {
            return pamac_string.to_owned();
        }

        package_name.to_owned()
    }

    fn install_packages(&self, packages: &[String], options: &install::Options) -> Result<()> {
        let mut command = std::process::Command::new(&self.executable_path);

        command.arg("install");

        // Now add all the translated package names
        for package in packages {
            command.arg(package);
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

    fn remove_packages(&self, packages: &[String], options: &remove::Options) -> Result<()> {
        let mut command = std::process::Command::new(&self.executable_path);

        command.arg("remove");

        // Now add all the translated package names
        for package in packages {
            command.arg(package);
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
