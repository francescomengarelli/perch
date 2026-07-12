use std::path::PathBuf;

pub fn expand_tilde(path: PathBuf) -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        if let Ok(stripped) = path.strip_prefix("~") {
            return PathBuf::from(home).join(stripped);
        }
    }
    path
}
