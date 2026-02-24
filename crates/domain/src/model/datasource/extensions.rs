//! Extension metadata (e.g. Postgres extensions). Mirrors `@domain/entities/datasource-meta` DatasourceExtension (Zod).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasourceExtension {
    pub name: String,
    pub schema: Option<String>,
    pub default_version: String,
    pub installed_version: Option<String>,
    pub comment: String,
}
