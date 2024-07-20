#[must_use]
pub fn file_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}
