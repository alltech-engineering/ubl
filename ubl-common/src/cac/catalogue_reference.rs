// UBL Catalogue Reference aggregate.
// A reference to a Catalogue document.

use serde::{Deserialize, Serialize};
use crate::cbc::*;

/// A reference to a Catalogue document on which a business document is based.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogueReference {
    pub id: ID,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<IssueDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    // TODO: RevisionDate, RevisionTime — not yet defined (use LastRevisionDate, IssueTime instead)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_revision_date: Option<LastRevisionDate>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<Description>,
}
