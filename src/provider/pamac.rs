use crate::application::Application;
use crate::prelude::*;
use crate::provider::Provider;
use crate::subcommand::install;
use crate::subcommand::remove;
use crate::utility::command_to_full_string;
use std::path::PathBuf;

// TODO: Instead of installed boolean just have executable_path as Option

#[allow(clippy::module_name_repetitions)]
pub struct PamacProvider {
    executable_path: PathBuf,
    installed: bool,
}

impl PamacProvider {
    fn run_command(
        &self,
        command_name: &str,
        packages: &[String],
        assume_yes: bool,
        dry_run: bool,
    ) -> Result<()> {
        let mut command = std::process::Command::new(&self.executable_path);

        command.arg(command_name);

        // Now add all the translated package names
        for package in packages {
            command.arg(package);
        }

        // Handle assume yes
        if assume_yes {
            command.arg("--no-confirm");
        }

        // Handle dry run
        if dry_run {
            command.arg("-d");
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

impl Provider for PamacProvider {
    fn name(&self) -> &'static str {
        "pamac"
    }

    fn initialize() -> Self {
        if cfg!(target_os = "linux") {
            which::which("pamac").map_or_else(
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
        if let Some(pamac_string) = &application.pamac {
            return pamac_string.to_owned();
        }

        package_name.to_owned()
    }

    fn install_packages(&self, packages: &[String], options: &install::Options) -> Result<()> {
        self.run_command("install", packages, options.assume_yes, options.dry_run)
    }

    fn remove_packages(&self, packages: &[String], options: &remove::Options) -> Result<()> {
        self.run_command("remove", packages, options.assume_yes, options.dry_run)
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

        // NOTE: pamac's reinstall can't handle dry run
        self.run_command("reinstall", packages, options.assume_yes, false)
    }

    fn update_packages(&self, options: &crate::subcommand::update::Options) -> Result<()> {
        self.run_command("update", &[], options.assume_yes, options.dry_run)
    }
}
