use assert_cmd::Command;
use cpkg::database::load_from_file;

#[test]
fn test_dry_install_all_packages() {
    let database = load_from_file("database.toml").unwrap();

    for package in database.packages.keys() {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        cmd.arg("install");
        cmd.arg(package);
        cmd.arg("--dry-run");

        cmd.assert().success();
    }
}
