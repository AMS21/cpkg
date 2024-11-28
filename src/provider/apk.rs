use crate::application::Application;
use crate::prelude::*;
use crate::provider::Provider;
use crate::subcommand::install;
use crate::superuser::get_superuser_launcher_path;
use crate::utility::command_to_full_string;
use is_superuser::is_superuser;
use std::path::PathBuf;

#[allow(clippy::module_name_repetitions)]
pub struct ApkProvider {
    executable_path: PathBuf,
    installed: bool,
}

// NOTE: apk doesn't support assume yes it will never ask for conformation.

impl ApkProvider {
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

    fn run_command(&self, command_name: &str, packages: &[String], dry_run: bool) -> Result<()> {
        let mut command = self.get_superuser_command()?;

        command.arg(command_name);

        // Now add all the translated package names
        for package in packages {
            command.arg(package);
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

impl Provider for ApkProvider {
    fn name(&self) -> &'static str {
        "apk"
    }

    fn initialize() -> Self {
        if cfg!(target_os = "linux") {
            if let Ok(apk_path) = which::which("apk") {
                return Self {
                    executable_path: apk_path,
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
        if let Some(apk_string) = &application.apk {
            return apk_string.to_string();
        }

        package_name.to_owned()
    }

    fn install_packages(&self, packages: &[String], options: &install::Options) -> Result<()> {
        // Update the package list first
        self.run_command("update", &[], false)?;

        self.run_command("add", packages, options.dry_run)
    }

    fn remove_packages(
        &self,
        packages: &[String],
        options: &crate::subcommand::remove::Options,
    ) -> Result<()> {
        // Update the package list first
        self.run_command("update", &[], false)?;

        self.run_command("del", packages, options.dry_run)
    }

    fn reinstall_packages(
        &self,
        packages: &[String],
        options: &crate::subcommand::reinstall::Options,
    ) -> Result<()> {
        // Update the package list first
        self.run_command("update", &[], false)?;

        // NOTE: apk doesn't support reinstalling so instead we delete and then add again the package
        self.run_command("del", packages, options.dry_run)?;

        self.run_command("add", packages, options.dry_run)
    }

    fn update_packages(&self, options: &crate::subcommand::update::Options) -> Result<()> {
        // Update the package list first
        self.run_command("update", &[], false)?;

        self.run_command("upgrade", &[], options.dry_run)
    }
}
