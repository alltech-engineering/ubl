#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentReference {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
    #[serde(default, rename = "DocumentTypeCode")]
    pub document_type_code: Option<cct::Code>,
    #[serde(default, rename = "DocumentType")]
    pub document_type: Vec<cct::Text>,
    #[serde(default, rename = "XPath")]
    pub x_path: Vec<cct::Text>,
    #[serde(default, rename = "ReferencedDocumentInternalAddress")]
    pub referenced_document_internal_address: Option<cct::Text>,
    #[serde(default, rename = "LanguageID")]
    pub language_id: Option<cct::Identifier>,
    #[serde(default, rename = "LocaleCode")]
    pub locale_code: Option<cct::Code>,
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
    #[serde(default, rename = "DocumentStatusCode")]
    pub document_status_code: Option<cct::Code>,
    #[serde(default, rename = "DocumentDescription")]
    pub document_description: Vec<cct::Text>,
    #[serde(default, rename = "Attachment")]
    pub attachment: Option<crate::Attachment>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<crate::Period>,
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: Option<crate::Party>,
    #[serde(default, rename = "ResultOfVerification")]
    pub result_of_verification: Option<crate::ResultOfVerification>,
}
