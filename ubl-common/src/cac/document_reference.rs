// DocumentReference — UBL CAC aggregate
// References another business document (invoice, order, etc.)
use crate::cbc::*;

/// A reference to another document.
/// UBL element: cac:DocumentReference
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DocumentReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<IssueDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type_code: Option<DocumentTypeCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<DocumentType>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub xpath: Vec<XPath>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_document_internal_address: Option<ReferencedDocumentInternalAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_id: Option<LanguageID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_code: Option<LocaleCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<VersionID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_status_code: Option<DocumentStatusCode>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub document_description: Vec<DocumentDescription>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Attachment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<Period>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_party: Option<Party>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_of_verification: Option<ResultOfVerification>,
}

/// UBL CAC AttachmentType — embedded or external document attachment
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Attachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_document_binary_object: Option<EmbeddedDocumentBinaryObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reference: Option<ExternalReference>,
}

/// UBL CAC ExternalReferenceType — URI or external reference
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExternalReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<Text>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub description: Vec<Description>,
}

/// UBL CAC ResultOfVerificationType — validation result
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResultOfVerification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator_id: Option<ValidatorID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_result_code: Option<ValidationResultCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_date: Option<ValidationDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_time: Option<ValidationTime>,
}

use super::period::Period;
use super::party::Party;
