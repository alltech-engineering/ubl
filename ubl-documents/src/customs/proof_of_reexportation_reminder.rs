use serde::{Deserialize, Serialize};

// Re-exports for convenience

/// UBL 2.5 ProofOfReexportationReminder document type.
/// Reminder for proof of re-export.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProofOfReexportationReminder {
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
    pub copy_indicator: Option<ubl_common::cbc::CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<ubl_common::cbc::UUID>,
    pub issue_date: ubl_common::cbc::IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<ubl_common::cbc::IssueTime>,
    pub procedure_code: ubl_common::cbc::ProcedureCode,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<ubl_common::cbc::Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<ubl_common::cbc::VersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub goods_item_passport_id: Option<ubl_common::cbc::GoodsItemPassportID>,
    pub proof_of_reexportation_request_document_reference:
        ProofOfReexportationRequestDocumentReference,
    pub importing_guarantor_party: ImportingGuarantorParty,
    pub exporting_guarantor_party: ExportingGuarantorParty,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub importing_customs_party: Option<ImportingCustomsParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer_endorsement: Option<IssuerEndorsement>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payment_terms: Vec<ubl_common::cac::PaymentTerms>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub goods_item_passport_counterfoil: Vec<ubl_common::cac::goods_item::GoodsItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_document_reference: Vec<AdditionalDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<ubl_common::cac::Signature>,
}

// ── Inline CAC types ──

/// UBL 2.5 ProofOfReexportationRequestDocumentReference — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProofOfReexportationRequestDocumentReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<ubl_common::cac::document_reference::DocumentReference>,
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

/// UBL ImportingCustomsParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportingCustomsParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL 2.5 IssuerEndorsement — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssuerEndorsement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ubl_common::cbc::ID>,
}

/// UBL 2.5 AdditionalDocumentReference — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionalDocumentReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<ubl_common::cac::document_reference::DocumentReference>,
}
