use serde::{Deserialize, Serialize};

// Re-exports for convenience

/// UBL 2.5 BusinessCard document type.
/// Trading capability information — who you are and what you do.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusinessCard {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<ubl_common::cbc::VersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_version_id: Option<ubl_common::cbc::PreviousVersionID>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub brief_description: Vec<ubl_common::cbc::BriefDescription>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<ubl_common::cac::Signature>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_party: Option<SenderParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver_party: Option<ReceiverParty>,
    pub business_party: BusinessParty,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub brochure_document_reference: Vec<BrochureDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_document_reference: Vec<AdditionalDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub business_capability: Vec<BusinessCapability>,
}

// ── Inline CAC types ──

/// UBL SenderParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SenderParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL ReceiverParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReceiverParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL BusinessParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusinessParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL 2.5 BrochureDocumentReference — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BrochureDocumentReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<ubl_common::cac::document_reference::DocumentReference>,
}

/// UBL 2.5 AdditionalDocumentReference — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionalDocumentReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<ubl_common::cac::document_reference::DocumentReference>,
}

/// UBL 2.5 BusinessCapability — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusinessCapability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ubl_common::cbc::ID>,
}
