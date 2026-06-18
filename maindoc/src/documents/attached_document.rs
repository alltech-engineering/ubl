#[derive(Debug, Deserialize, Serialize)]
pub struct AttachedDocument {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::TextType>,
    #[serde(default, rename = "DocumentTypeCode")]
    pub document_type_code: Option<cct::CodeType>,
    #[serde(default, rename = "DocumentType")]
    pub document_type: Option<cct::TextType>,
    #[serde(rename = "ParentDocumentID")]
    pub parent_document_id: cct::IdentifierType,
    #[serde(default, rename = "ParentDocumentTypeCode")]
    pub parent_document_type_code: Option<cct::CodeType>,
    #[serde(default, rename = "ParentDocumentVersionID")]
    pub parent_document_version_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::Party,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::Party,
    #[serde(rename = "Attachment")]
    pub attachment: cac::Attachment,
    #[serde(default, rename = "ParentDocumentLineReference")]
    pub parent_document_line_reference: Vec<cac::LineReference>,
}
