use serde::{Deserialize, Serialize};

// Re-exports for convenience

/// UBL 2.5 InvoiceStatusResponse document type.
/// A response about invoice processing status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvoiceStatusResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<ubl_common::cbc::UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<ubl_common::cbc::CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ubl_common::cbc::ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ubl_common::cbc::ProfileExecutionID>,
    pub id: ubl_common::cbc::ID,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<ubl_common::cbc::UUID>,
    pub issue_date: ubl_common::cbc::IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<ubl_common::cbc::IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<ubl_common::cbc::Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<ubl_common::cbc::VersionID>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<ubl_common::cac::Signature>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_party: Option<SenderParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver_party: Option<ReceiverParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payment: Vec<ubl_common::cac::Payment>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document_response: Vec<ubl_common::cac::DocumentResponse>,
}

// ── Inline CAC types ──

/// UBL SenderParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SenderParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
    // TODO: Add role-specific fields from UBL 2.5 CAC schema
}

/// UBL ReceiverParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReceiverParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
    // TODO: Add role-specific fields from UBL 2.5 CAC schema
}
