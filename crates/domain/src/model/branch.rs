use serde::{Deserialize, Serialize};

use crate::model::ref_info::RefInfo;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BranchInfo {
    pub current: bool,
    pub ref_info: RefInfo,
}
