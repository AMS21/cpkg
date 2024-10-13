use assert_cmd::Command;
use cpkg::database::load_from_file;
use cpkg::provider::get_number_of_installed_providers;

#[test]
fn all_packages_are_installable() {
    // No point in running this tests if no providers are installed
    if get_number_of_installed_providers() == 0 {
        return;
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
