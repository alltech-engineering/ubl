#[derive(Debug, Deserialize, Serialize)]
/// A class to define a reference to a document.
///
/// UBL Dictionary Entry Name: `Document Reference. Details`
///
/// Generated from XSD type `DocumentReferenceType`.
pub struct DocumentReference {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for the referenced document.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// (Deprecated) An indicator that the referenced document is a copy (true) or the original (false).
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
/// A universally unique identifier for this document reference.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// The date, assigned by the sender of the referenced document, on which the document was issued.
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
/// The time, assigned by the sender of the referenced document, at which the document was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// The type of document being referenced, expressed as a code.
    #[serde(default, rename = "DocumentTypeCode")]
    pub document_type_code: Option<cct::Code>,
/// The type of document being referenced, expressed as text.
    #[serde(default, rename = "DocumentType")]
    pub document_type: Vec<cct::Text>,
/// An unambiguous location within the bounding document or the document referenced by the parent
/// DocumentReference, expressed as an XPath
    #[serde(default, rename = "XPath")]
    pub x_path: Vec<cct::Text>,
/// A pointer to a location within the document being referenced
    #[serde(default, rename = "ReferencedDocumentInternalAddress")]
    pub referenced_document_internal_address: Option<cct::Text>,
/// An identifier for the language used in the referenced document.
    #[serde(default, rename = "LanguageID")]
    pub language_id: Option<cct::Identifier>,
/// A code signifying the locale in which the language in the referenced document is used.
    #[serde(default, rename = "LocaleCode")]
    pub locale_code: Option<cct::Code>,
/// An identifier for the current version of the referenced document.
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// A code signifying the status of the reference document with respect to its original state.
    #[serde(default, rename = "DocumentStatusCode")]
    pub document_status_code: Option<cct::Code>,
/// Text describing the referenced document.
    #[serde(default, rename = "DocumentDescription")]
    pub document_description: Vec<cct::Text>,
/// The referenced document as an attachment to the document from which it is referenced.
    #[serde(default, rename = "Attachment")]
    pub attachment: Option<crate::Attachment>,
/// The period for which the document referenced by this Document Rreference is valid.
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<crate::Period>,
/// The Party who issues the Referenced Document.
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: Option<crate::Party>,
/// The result of an attempt to verify a signature associated with the referenced document.
    #[serde(default, rename = "ResultOfVerification")]
    pub result_of_verification: Option<crate::ResultOfVerification>,
}
