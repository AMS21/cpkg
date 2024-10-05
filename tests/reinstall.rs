use assert_cmd::Command;
use cpkg::provider::get_number_of_installed_providers;

#[test]
fn reinstall_non_existing_package() {
    // No point in running this tests if no providers are installed
    if get_number_of_installed_providers() == 0 {
        return;
    }

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    cmd.arg("reinstall");
    cmd.arg("This-Package-Should-Not-Exists");

    cmd.assert().failure();
}
