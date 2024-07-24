use crate::application::Application;
use crate::prelude::*;
use crate::provider::Provider;
use crate::subcommand::install;
use std::path::PathBuf;
use which::which;

#[allow(clippy::module_name_repetitions)]
pub struct AptProvider {
    executable_path: PathBuf,
    installed: bool,
}

impl Provider for AptProvider {
    fn name(&self) -> &'static str {
        "apt"
    }

    fn initialize() -> Self {
        which("apt").map_or_else(
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
        if let Some(apt_string) = &application.apt {
            return apt_string.to_string();
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

        // Don't install recommended packages
        command.arg("--no-install-recommends");

        // Add -y if assume_yes is true
        if options.assume_yes {
            command.arg("-y");
        }

        // Handle dry run
        if options.dry_run {
            command.arg("-s");
        }

        // Run the actual command
        let return_code = command.spawn()?.wait()?;

        if !return_code.success() {
            return Err(Error::InstallCommandFailed(return_code));
        }

        Ok(())
    }

    fn remove_packages(
        &self,
        packages: &[String],
        options: &crate::subcommand::remove::Options,
    ) -> Result<()> {
        let mut command = std::process::Command::new(&self.executable_path);

        command.arg("remove");

        // Now add all the translated package names
        for package in packages {
            command.arg(package);
        }

        // Add -y if assume_yes is true
        if options.assume_yes {
            command.arg("-y");
        }

        // Handle dry run
        if options.dry_run {
            command.arg("-s");
        }

        // Run the actual command
        let return_code = command.spawn()?.wait()?;

        if !return_code.success() {
            return Err(Error::RemoveCommandFailed(return_code));
        }

        Ok(())
    }
}
