use std::path::PathBuf;
use which::which;

#[must_use]
pub fn get_superuser_launcher_path() -> Option<PathBuf> {
    if let Ok(sudo_path) = which("sudo") {
        return Some(sudo_path);
    }

    if let Ok(doas_path) = which("doas") {
        return Some(doas_path);
    }

    if let Ok(run0_path) = which("run0") {
        return Some(run0_path);
    }

    None
}
