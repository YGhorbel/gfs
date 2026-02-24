//! Datasource version info.
//! Mirrors `@domain/entities/datasource-meta` DatasourceVersion (Zod).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasourceVersion {
    pub version: String,
    pub version_number: i64,
    pub active_connections: i64,
    pub max_connections: i64,
}
