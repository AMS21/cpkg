use crate::application::Application;
use crate::prelude::*;
use crate::provider::Provider;
use crate::subcommand::install;
use crate::superuser::get_superuser_launcher_path;
use crate::utility::command_to_full_string;
use is_superuser::is_superuser;
use std::path::PathBuf;

#[allow(clippy::module_name_repetitions)]
pub struct AptProvider {
    executable_path: PathBuf,
    installed: bool,
}

impl AptProvider {
    fn get_superuser_command(&self) -> Result<std::process::Command> {
        // if we are root then we don't need to use any superuser launcher
        if is_superuser() {
            return Ok(std::process::Command::new(&self.executable_path));
        }

        // Otherwise we need to use the superuser launcher
        let superuser_launcher =
            get_superuser_launcher_path().ok_or_else(|| Error::NoSuperuserLauncherFound {
                provider: self.name(),
            })?;

        let mut command = std::process::Command::new(superuser_launcher);

        // Set the executable as the first argument
        command.arg(&self.executable_path);

        Ok(command)
    }

    fn run_command(
        &self,
        command_name: &str,
        packages: &[String],
        assume_yes: bool,
        dry_run: bool,
    ) -> Result<()> {
        let mut command = self.get_superuser_command()?;

        command.arg(command_name);

        // Now add all the translated package names
        for package in packages {
            command.arg(package);
        }

        // Add -y if assume_yes is true
        if assume_yes {
            command.arg("-y");
        }

        // Handle dry run
        if dry_run {
            command.arg("-s");
        }

        // Run the actual command
        let return_code = command.spawn()?.wait()?;

        if !return_code.success() {
            return Err(Error::CommandFailed {
                exit_code: return_code,
                command_line: command_to_full_string(&command)?,
            });
        }

        Ok(())
    }
}

impl Provider for AptProvider {
    fn name(&self) -> &'static str {
        "apt"
    }

    fn initialize() -> Self {
        if cfg!(target_os = "linux") {
            // First search for 'apt'
            if let Ok(apt_path) = which::which("apt") {
                return Self {
                    executable_path: apt_path,
                    installed: true,
                };
            }

            // Then search for 'apt-get'
            if let Ok(apt_get_path) = which::which("apt-get") {
                return Self {
                    executable_path: apt_get_path,
                    installed: true,
                };
            }
        }

        Self {
            executable_path: PathBuf::new(),
            installed: false,
        }
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
        // Update the package list first
        self.run_command("update", &[], false, false)?;

        self.run_command("install", packages, options.assume_yes, options.dry_run)
    }

    fn remove_packages(
        &self,
        packages: &[String],
        options: &crate::subcommand::remove::Options,
    ) -> Result<()> {
        // Update the package list first
        self.run_command("update", &[], false, false)?;

        self.run_command("remove", packages, options.assume_yes, options.dry_run)
    }

    fn reinstall_packages(
        &self,
        packages: &[String],
        options: &crate::subcommand::reinstall::Options,
    ) -> Result<()> {
        // Update the package list first
        self.run_command("update", &[], false, false)?;

        self.run_command("reinstall", packages, options.assume_yes, options.dry_run)
    }

    fn update_packages(&self, options: &crate::subcommand::update::Options) -> Result<()> {
        // Update the package list first
        self.run_command("update", &[], false, false)?;

        self.run_command("upgrade", &[], options.assume_yes, options.dry_run)
    }
}
