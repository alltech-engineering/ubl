// DocumentReference — UBL CAC aggregate
// References another business document (invoice, order, etc.)
use crate::cbc::*;

/// A reference to another document.
/// UBL element: cac:DocumentReference
#[derive(Debug, Clone, Partialserde::Serialize, serde::Deserialize)]
pub struct DocumentReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<IssueDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type_code: Option<DocumentTypeCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<DocumentType>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub xpath: Vec<XPath>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_id: Option<LanguageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_code: Option<LocaleCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<VersionId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_status_code: Option<DocumentStatusCode>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub document_description: Vec<DocumentDescription>,
}
