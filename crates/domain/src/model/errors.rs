use bincode;
use serde_json;
use std::io;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum CommitError {
    #[error("Failed to create commit: {0}")]
    CreationError(String),

    #[error("Failed to write commit object: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Failed to serialize commit: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Failed to decode commit: {0}")]
    DecodeError(#[from] bincode::error::DecodeError),

    #[error("Failed to encode commit: {0}")]
    EncodeError(#[from] bincode::error::EncodeError),
    /*
    #[error("Invalid commit message: {0}")]
    InvalidMessage(String),

    #[error("Failed to generate commit hash: {0}")]
    HashError(String),

    #[error("Failed to update branch reference: {0}")]
    BranchUpdateError(String),
     */
}

#[derive(Debug, thiserror::Error)]
pub enum RepoError {
    #[error("No .gfs repository found in {0} or any parent directory")]
    NoRepoFound(PathBuf),
    #[error("IO error while searching for repository: {0}")]
    IoError(#[from] io::Error),
    #[error("Invalid repository layout: {0}")]
    InvalidLayout(String),
    #[error("Missing required file: {0}")]
    MissingFile(PathBuf),
    #[error("Invalid config.toml: {0}")]
    InvalidConfig(String),
    #[error("revision not found: '{0}'")]
    RevisionNotFound(String),
}

impl RepoError {
    pub fn no_repo_found(path: PathBuf) -> Self {
        RepoError::NoRepoFound(path)
    }

    pub fn invalid_layout(msg: String) -> Self {
        RepoError::InvalidLayout(msg)
    }

    pub fn missing_file(path: PathBuf) -> Self {
        RepoError::MissingFile(path)
    }
}
