use std::path::{Path, PathBuf};

/// Returns the current working directory as the default repo path.
pub fn get_repo_dir() -> PathBuf {
    std::env::current_dir().expect("current directory not available")
}

/// Return a path relative to the repo root (e.g. `.gfs/workspaces/dev/.../data`).
/// If relativization fails, returns the original path string.
pub fn relativize_to_repo(repo_path: &Path, full_path: &str) -> String {
    let repo = match repo_path.canonicalize() {
        Ok(p) => p,
        Err(_) => repo_path.to_path_buf(),
    };
    let full = PathBuf::from(full_path);
    let full_canon = match full.canonicalize() {
        Ok(p) => p,
        Err(_) => full,
    };
    full_canon
        .strip_prefix(&repo)
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| full_path.to_string())
}
