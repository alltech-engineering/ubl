// OrderLine — UBL CAC aggregate
// A line in an Order document.
use crate::cbc::*;

/// A line in an order.
/// UBL element: cac:OrderLine
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitution_status_code: Option<Code>,
}

// Placeholder Code type alias for substitution_status_code
pub type Code = crate::cbc::LineStatusCode;
