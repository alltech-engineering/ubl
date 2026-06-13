// UBL Catalogue Reference aggregate.
// A reference to a Catalogue document.

use crate::cbc::*;
use serde::{Deserialize, Serialize};

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_date: Option<RevisionDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_time: Option<RevisionTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<Description>,
}
