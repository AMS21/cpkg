use crate::application::Application;
use crate::prelude::*;
use crate::provider::Provider;
use crate::subcommand::install;
use crate::utility::command_to_full_string;
use std::path::PathBuf;

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
        if cfg!(target_os = "linux") {
            which::which("apt").map_or_else(
                |_| Self {
                    executable_path: PathBuf::new(),
                    installed: false,
                },
                |apt_path| Self {
                    executable_path: apt_path,
                    installed: true,
                },
            )
        } else {
            Self {
                executable_path: PathBuf::new(),
                installed: false,
            }
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
            return Err(Error::InstallCommandFailed {
                exit_code: return_code,
                command_line: command_to_full_string(&command)?,
            });
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
            return Err(Error::RemoveCommandFailed {
                exit_code: return_code,
                command_line: command_to_full_string(&command)?,
            });
        }

        Ok(())
    }

    fn reinstall_packages(
        &self,
        packages: &[String],
        options: &crate::subcommand::reinstall::Options,
    ) -> Result<()> {
        let mut command = std::process::Command::new(&self.executable_path);

        command.arg("reinstall");

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
            return Err(Error::ReinstallCommandFailed {
                exit_code: return_code,
                command_line: command_to_full_string(&command)?,
            });
        }

        Ok(())
    }
}
