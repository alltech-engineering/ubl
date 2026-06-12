// UBL Transaction Conditions aggregate.
// Conditions of sale, delivery, or payment that apply to a transaction.

use serde::{Deserialize, Serialize};
use crate::cbc::*;
use crate::cac::document::DocumentReference;

/// Conditions that apply to the whole transaction (sale, delivery, payment).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionConditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_code: Option<ActionCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<Description>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document_reference: Vec<DocumentReference>,
}
