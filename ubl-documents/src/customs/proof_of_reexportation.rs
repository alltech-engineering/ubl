use serde::{Deserialize, Serialize};

// Re-exports for convenience

/// UBL 2.5 ProofOfReexportation document type.
/// Proof of re-export of goods.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProofOfReexportation {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<ubl_common::cbc::IssueDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<ubl_common::cbc::IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<ubl_common::cbc::Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<ubl_common::cbc::VersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exporting_customs_party: Option<ExportingCustomsParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub importing_guarantor_party: Option<ImportingGuarantorParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exporting_guarantor_party: Option<ExportingGuarantorParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub goods_item_passport_counterfoil: Vec<ubl_common::cac::goods_item::GoodsItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reexportation_evidence: Vec<ReexportationEvidence>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub goods_item_passport_attachment: Option<ubl_common::cac::Attachment>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_document_reference: Vec<AdditionalDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<ubl_common::cac::Signature>,
}

// ── Inline CAC types ──

/// UBL ExportingCustomsParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportingCustomsParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL ImportingGuarantorParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportingGuarantorParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL ExportingGuarantorParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportingGuarantorParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL 2.5 ReexportationEvidence — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReexportationEvidence {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ubl_common::cbc::ID>,
}

/// UBL 2.5 AdditionalDocumentReference — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionalDocumentReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<ubl_common::cac::document_reference::DocumentReference>,
}
