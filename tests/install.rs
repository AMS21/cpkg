use assert_cmd::Command;
use cpkg::provider::get_number_of_installed_providers;

#[test]
fn install_non_existing_package() {
    // No point in running this tests if no providers are installed
    if get_number_of_installed_providers() == 0 {
        return;
    }

    let mut cmd = Command::cargo_bin("cpkg").unwrap();

    cmd.arg("install");
    cmd.arg("This-Package-Should-Not-Exists");

    cmd.assert().failure();
}
