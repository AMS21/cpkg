use assert_cmd::Command;
use cpkg::database::load_from_file;
use cpkg::provider::get_all_providers;

#[test]
fn all_packages_are_dry_installable() {
    let providers = get_all_providers();

    // No point in running this tests if no providers are installed
    if providers.is_empty() {
        return;
    }

    // The 'dnf' package manager cannot handle dry runs so skip the test if it's installed since it will cause an error here
    for provider in providers {
        if provider.name() == "dnf" {
            return;
        }
    }

    let database = load_from_file("database.toml").unwrap();

    for package in database.packages.keys() {
        let mut cmd = Command::cargo_bin("cpkg").unwrap();

        cmd.arg("install");
        cmd.arg(package);
        cmd.arg("--dry-run");

        cmd.assert().success();
    }
}

// NOTE: See the note in Cargo.toml for the need of the '_run_actual_installs_when_testing' feature
//       and why you probably don't want to run this test yourself.
#[test]
#[cfg(all(feature = "_run_actual_installs_when_testing", feature = "dnf"))]
fn all_packages_are_installable() {
    let providers = get_all_providers();

    // Only run this test if the 'dnf' package manager is installed
    if providers.len() != 1 || providers[0].name() != "dnf" {
        return;
    }

    let database = load_from_file("database.toml").unwrap();

    for package in database.packages.keys() {
        let mut cmd = Command::cargo_bin("cpkg").unwrap();

        cmd.arg("install");
        cmd.arg(package);
        cmd.arg("-y");

        cmd.assert().success();
    }
}
