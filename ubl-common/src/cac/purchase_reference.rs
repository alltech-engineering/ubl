// UBL Purchase Reference aggregate.
// A reference to a Purchase Order from a line-level document.
//
// UBL element: cac:PurchaseReference

use serde::{Deserialize, Serialize};
use crate::cbc::*;

/// A reference to a purchase order (used for purchase-based lines).
/// UBL element: cac:PurchaseReference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<IssueDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
}
