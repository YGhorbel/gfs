//! Schema (namespace) metadata for datasources.
//! Mirrors `@domain/entities/datasource-meta` Schema (Zod).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub id: i64,
    pub name: String,
    pub owner: String,
}
