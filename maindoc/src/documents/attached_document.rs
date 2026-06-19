#[derive(Debug, Deserialize, Serialize)]
/// (Deprecated) A wrapper that allows a document of any kind to be packaged with the UBL document that
/// references it.
///
/// UBL Dictionary Entry Name: `Attached Document. Details`
///
/// Generated from XSD type `AttachedDocumentType`.
pub struct AttachedDocument {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
/// Identifies the earliest version of the UBL 2 schema for this document type that defines all of the
/// elements that might be encountered in the current instance.
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::Identifier>,
/// Identifies a user-defined customization of UBL for a specific use.
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::Identifier>,
/// Identifies a user-defined profile of the customization of UBL being used.
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::Identifier>,
/// Identifies an instance of executing a profile, to associate all transactions in a collaboration.
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::Identifier>,
/// An identifier for this document, assigned by the sender.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// A universally unique identifier for an instance of this document.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// The date, assigned by the sender, on which this document was issued.
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
/// The time, assigned by the sender, at which this document was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// A code signifying the type of document.
    #[serde(default, rename = "DocumentTypeCode")]
    pub document_type_code: Option<cct::Code>,
/// Text specifying the type of document.
    #[serde(default, rename = "DocumentType")]
    pub document_type: Option<cct::Text>,
/// The Identifier of the parent document.
    #[serde(rename = "ParentDocumentID")]
    pub parent_document_id: cct::Identifier,
/// A code signifying the type of parent document.
    #[serde(default, rename = "ParentDocumentTypeCode")]
    pub parent_document_type_code: Option<cct::Code>,
/// Indicates the current version of the referred document.
    #[serde(default, rename = "ParentDocumentVersionID")]
    pub parent_document_version_id: Option<cct::Identifier>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The Party who sends this Document.
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::Party,
/// The Party who receives this Document.
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::Party,
/// An attachment containing the document content.
    #[serde(rename = "Attachment")]
    pub attachment: cac::Attachment,
/// A reference to a line in the attached document.
    #[serde(default, rename = "ParentDocumentLineReference")]
    pub parent_document_line_reference: Vec<cac::LineReference>,
}
