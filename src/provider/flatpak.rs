use crate::application::Application;
use crate::application::Value;
use crate::lookup::LookupResult;
use crate::prelude::*;
use crate::provider::Provider;
use crate::subcommand::install;
use crate::subcommand::remove;
use crate::utility::command_to_full_string;
use std::path::PathBuf;

#[expect(clippy::module_name_repetitions)]
pub struct FlatpakProvider {
    executable_path: PathBuf,
    installed: bool,
}

impl FlatpakProvider {
    fn run_command(
        &self,
        command_name: &'static str,
        packages: &[String],
        assume_yes: bool,
    ) -> Result<()> {
        let mut command = std::process::Command::new(&self.executable_path);

        command.arg(command_name);

        // Now add all the translated package names
        for package in packages {
            command.arg(package);
        }

        // Handle assume yes
        if assume_yes {
            command.arg("--assumeyes");
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

impl Provider for FlatpakProvider {
    fn name(&self) -> &'static str {
        "flatpak"
    }

    fn initialize() -> Self {
        if cfg!(target_os = "linux") {
            which::which("flatpak").map_or_else(
                |_| Self {
                    executable_path: PathBuf::new(),
                    installed: false,
                },
                |flatpak_path| Self {
                    executable_path: flatpak_path,
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

    fn lookup_package(&self, application: &Application, package_name: &str) -> LookupResult {
        match &application.flatpak {
            Some(Value::String(string)) => LookupResult::InstallWith(string.clone()),
            Some(Value::Bool(true)) => LookupResult::AlwaysInstalled,
            Some(Value::Bool(false)) => LookupResult::NeverInstalled,
            Some(Value::Object(_object)) => LookupResult::NeverInstalled,
            None => LookupResult::InstallWith(package_name.to_owned()),
        }
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

    fn remove_packages(&self, packages: &[String], options: &remove::Options) -> Result<()> {
        if options.dry_run {
            return Err(Error::OptionNotSupported {
                option_name: "dry run",
                operation: "remove",
                provider: self.name(),
            });
        }

        self.run_command("uninstall", packages, options.assume_yes)
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

        self.run_command("uninstall", packages, options.assume_yes)?;
        self.run_command("install", packages, options.assume_yes)
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
