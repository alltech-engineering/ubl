use serde::{Deserialize, Serialize};

// Re-exports for convenience

/// UBL 2.5 BusinessInformation document type.
/// Business registration/notification information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusinessInformation {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_version_id: Option<ubl_common::cbc::PreviousVersionID>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub brief_description: Vec<ubl_common::cbc::BriefDescription>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_publication_date: Option<ubl_common::cbc::RequestedPublicationDate>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub regulatory_domain: Vec<ubl_common::cbc::RegulatoryDomain>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notice_type_code: Option<ubl_common::cbc::NoticeTypeCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notice_language_code: Option<ubl_common::cbc::NoticeLanguageCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_notice_language: Vec<AdditionalNoticeLanguage>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub business_party_group: Vec<BusinessPartyGroup>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operation_type: Vec<OperationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notice_sub_type: Option<NoticeSubType>,
}

// ── Inline CAC types ──

/// UBL 2.5 AdditionalNoticeLanguage — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionalNoticeLanguage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ubl_common::cbc::ID>,
}

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

/// UBL BusinessPartyGroup — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusinessPartyGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL 2.5 OperationType — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OperationType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<ubl_common::cbc::Code>,
}

/// UBL 2.5 NoticeSubType — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoticeSubType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_type_code: Option<ubl_common::cbc::SubTypeCode>,
}
