// UBL Project Reference aggregate.
// A reference to a project to which a business document relates.

use crate::cbc::*;
use serde::{Deserialize, Serialize};

/// A reference to a project.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectReference {
    pub id: ID,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<IssueDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_phase_reference: Option<WorkPhaseReference>,
}

/// A reference to a phase of work within a project.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WorkPhaseReference {
    pub id: Option<ID>,
    pub uuid: Option<UUID>,
    pub issue_date: Option<IssueDate>,
    pub issue_time: Option<IssueTime>,
    pub work_phase_code: Option<WorkPhaseCode>,
    pub progress_percent: Option<ProgressPercent>,
}
