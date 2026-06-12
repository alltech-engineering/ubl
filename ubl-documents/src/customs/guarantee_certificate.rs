use serde::{Deserialize, Serialize};

// Re-exports for convenience

/// UBL 2.5 GuaranteeCertificate document type.
/// Financial guarantee certificate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuaranteeCertificate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<ubl_common::cbc::UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<ubl_common::cbc::CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ubl_common::cbc::ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ubl_common::cbc::ProfileExecutionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ubl_common::cbc::ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<ubl_common::cbc::CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<ubl_common::cbc::UUID>,
    pub contract_folder_id: ubl_common::cbc::ContractFolderID,
    pub issue_date: ubl_common::cbc::IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<ubl_common::cbc::IssueTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guarantee_type_code: Option<ubl_common::cbc::GuaranteeTypeCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub purpose: Vec<ubl_common::cbc::Purpose>,
    pub liability_amount: ubl_common::cbc::LiabilityAmount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constitution_code: Option<ubl_common::cbc::ConstitutionCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<ubl_common::cbc::Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applicable_period: Option<ApplicablePeriod>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applicable_regulation: Vec<ApplicableRegulation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guarantee_document_reference: Vec<GuaranteeDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub immobilized_security: Vec<ImmobilizedSecurity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<ubl_common::cac::Signature>,
    pub guarantor_party: GuarantorParty,
    pub interested_party: InterestedParty,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub beneficiary_party: Option<BeneficiaryParty>,
}

// ── Inline CAC types ──

/// UBL ApplicablePeriod — a Period with this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicablePeriod {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<ubl_common::cac::Period>,
}

/// UBL 2.5 ApplicableRegulation — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicableRegulation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ubl_common::cbc::Name>,
}

/// UBL 2.5 GuaranteeDocumentReference — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuaranteeDocumentReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<ubl_common::cac::document_reference::DocumentReference>,
}

/// UBL 2.5 ImmobilizedSecurity — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImmobilizedSecurity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ubl_common::cbc::ID>,
}

/// UBL GuarantorParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuarantorParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL InterestedParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterestedParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL BeneficiaryParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeneficiaryParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}
