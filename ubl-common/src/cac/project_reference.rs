// UBL Project Reference aggregate.
// A reference to a project to which a business document relates.

use serde::{Deserialize, Serialize};
use crate::cbc::*;

/// A reference to a project.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectReference {
    pub id: ID,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<IssueDate>,
    // TODO: WorkPhaseReference — not yet defined as a CBC type
}
