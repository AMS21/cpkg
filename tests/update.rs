use assert_cmd::Command;
use cpkg::provider::get_installed_providers;

#[test]
fn dry_update_all_packages() {
    let providers = get_installed_providers();

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

    let mut cmd = Command::cargo_bin("cpkg").unwrap();

    cmd.arg("update");
    cmd.arg("--dry-run");

    cmd.assert().success();
}

// NOTE: See the note in Cargo.toml for the need of the '_run_actual_installs_when_testing' feature
//       and why you probably don't want to run this test yourself.
#[test]
#[cfg(all(feature = "_run_actual_installs_when_testing", feature = "dnf"))]
fn update_all_packages() {
    let providers = get_installed_providers();

    // Only run this test if the 'dnf' package manager is installed
    if providers.len() != 1 || providers[0].name() != "dnf" {
        return;
    }

    let mut cmd = Command::cargo_bin("cpkg").unwrap();

    cmd.arg("update");
    cmd.arg("-y");

    cmd.assert().success();
}
