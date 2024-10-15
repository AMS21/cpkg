use crate::application::Application;
use crate::prelude::*;
use crate::provider::Provider;
use crate::subcommand::install;
use crate::utility::command_to_full_string;
use std::path::PathBuf;

#[allow(clippy::module_name_repetitions)]
pub struct DnfProvider {
    executable_path: PathBuf,
    installed: bool,
}

// NOTE: dnf cannot handle dry run for any operation

impl DnfProvider {
    fn run_command(&self, command_name: &str, packages: &[String], assume_yes: bool) -> Result<()> {
        let mut command = std::process::Command::new(&self.executable_path);

        command.arg(command_name);

        // Now add all the translated package names
        for package in packages {
            command.arg(package);
        }

        // Add -y if assume_yes is true
        if assume_yes {
            command.arg("-y");
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

impl Provider for DnfProvider {
    fn name(&self) -> &'static str {
        "dnf"
    }

    fn initialize() -> Self {
        if cfg!(target_os = "linux") {
            // First check for 'dnf'
            if let Ok(dnf_path) = which::which("dnf") {
                return Self {
                    executable_path: dnf_path,
                    installed: true,
                };
            }

            // Then check for 'microdnf'
            // https://github.com/rpm-software-management/microdnf
            if let Ok(microdnf_path) = which::which("microdnf") {
                return Self {
                    executable_path: microdnf_path,
                    installed: true,
                };
            }

            // Finally check for 'tdnf'
            // https://github.com/vmware/tdnf
            if let Ok(tdnf_path) = which::which("tdnf") {
                return Self {
                    executable_path: tdnf_path,
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
        if let Some(dnf_string) = &application.dnf {
            return dnf_string.to_string();
        }

        package_name.to_owned()
    }

    fn install_packages(&self, packages: &[String], options: &install::Options) -> Result<()> {
        if options.dry_run {
            return Err(Error::OptionNotSupported {
                option_name: "dry run",
                operation: "install",
                provider: self.name(),
            });
        }

        self.run_command("install", packages, options.assume_yes)
    }

    fn remove_packages(
        &self,
        packages: &[String],
        options: &crate::subcommand::remove::Options,
    ) -> Result<()> {
        if options.dry_run {
            return Err(Error::OptionNotSupported {
                option_name: "dry run",
                operation: "remove",
                provider: self.name(),
            });
        }

        self.run_command("remove", packages, options.assume_yes)
    }

    fn reinstall_packages(
        &self,
        packages: &[String],
        options: &crate::subcommand::reinstall::Options,
    ) -> Result<()> {
        if options.dry_run {
            return Err(Error::OptionNotSupported {
                option_name: "dry run",
                operation: "reinstall",
                provider: self.name(),
            });
        }

        self.run_command("reinstall", packages, options.assume_yes)
    }

    fn update_packages(&self, options: &crate::subcommand::update::Options) -> Result<()> {
        if options.dry_run {
            return Err(Error::OptionNotSupported {
                option_name: "dry run",
                operation: "update",
                provider: self.name(),
            });
        }

        self.run_command("update", &[], options.assume_yes)
    }
}
