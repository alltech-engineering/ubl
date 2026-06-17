use serde::{Deserialize, Serialize};

// Re-exports for convenience

/// UBL 2.5 DigitalAgreement document type.
/// Trading partner agreement for digital exchange.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalAgreement {
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
    pub agreement_type_code: Option<ubl_common::cbc::AgreementTypeCode>,
    pub version_id: ubl_common::cbc::VersionID,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_version_id: Option<ubl_common::cbc::PreviousVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_response_message_level_code:
        Option<ubl_common::cbc::RequiredResponseMessageLevelCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<ubl_common::cac::Signature>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub governor_party: Option<GovernorParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant_party: Vec<ParticipantParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub agreement_country: Vec<AgreementCountry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_certification_document_reference: Vec<RequiredCertificationDocumentReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digital_agreement_terms: Option<DigitalAgreementTerms>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub digital_process: Vec<DigitalProcess>,
}

// ── Inline CAC types ──

/// UBL GovernorParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GovernorParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL ParticipantParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParticipantParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL 2.5 AgreementCountry — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgreementCountry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<ubl_common::cac::address::Country>,
}

/// UBL 2.5 RequiredCertificationDocumentReference — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequiredCertificationDocumentReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<ubl_common::cac::document_reference::DocumentReference>,
}

/// UBL 2.5 DigitalAgreementTerms — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalAgreementTerms {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<ubl_common::cbc::Description>,
}

/// UBL 2.5 DigitalProcess — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalProcess {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ubl_common::cbc::ID>,
}
