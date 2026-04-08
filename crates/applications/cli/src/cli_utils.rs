use std::path::{Path, PathBuf};

use anyhow::Result;
use gfs_domain::model::layout::{GFS_DIR, HEADS_DIR, REFS_DIR};

/// Returns the current working directory as the default repo path.
pub fn get_repo_dir() -> PathBuf {
    std::env::current_dir().expect("current directory not available")
}

/// Collect branch tips: Vec<(branch_name, commit_hash)>.
///
/// When `missing_ok` is true and the refs directory doesn't exist, returns an empty list.
pub fn list_branch_tips(repo_path: &Path, missing_ok: bool) -> Result<Vec<(String, String)>> {
    let refs_dir = repo_path.join(GFS_DIR).join(REFS_DIR).join(HEADS_DIR);
    if !refs_dir.exists() {
        if missing_ok {
            return Ok(Vec::new());
        }
        anyhow::bail!("not a GFS repository (no refs directory)");
    }
    collect_refs(&refs_dir, "")
}

fn collect_refs(dir: &Path, prefix: &str) -> Result<Vec<(String, String)>> {
    let mut result = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        let branch_name = if prefix.is_empty() {
            name
        } else {
            format!("{}/{}", prefix, name)
        };
        if path.is_file() {
            let tip = std::fs::read_to_string(&path)?.trim().to_string();
            result.push((branch_name, tip));
        } else if path.is_dir() {
            result.extend(collect_refs(&path, &branch_name)?);
        }
    }
    Ok(result)
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
