use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum RefKind {
    LocalBranch,
    RemoteBranch,
    Tag,
    Stash,
    Notes,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefInfo {
    pub name: String,
    pub kind: RefKind,
    pub oid: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub timestamp: DateTime<Utc>,
    pub author: Option<String>,
    pub subject: Option<String>,
}
