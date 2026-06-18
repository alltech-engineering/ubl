#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentReference {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "DocumentTypeCode")]
    pub document_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "DocumentType")]
    pub document_type: Vec<super::cct::TextType>,
    #[serde(default, rename = "XPath")]
    pub x_path: Vec<super::cct::TextType>,
    #[serde(default, rename = "ReferencedDocumentInternalAddress")]
    pub referenced_document_internal_address: Option<super::cct::TextType>,
    #[serde(default, rename = "LanguageID")]
    pub language_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "LocaleCode")]
    pub locale_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "DocumentStatusCode")]
    pub document_status_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "DocumentDescription")]
    pub document_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "Attachment")]
    pub attachment: Option<Attachment>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<Period>,
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: Option<Party>,
    #[serde(default, rename = "ResultOfVerification")]
    pub result_of_verification: Option<ResultOfVerification>,
}
