#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentMetadata {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(rename = "FormatID")]
    pub format_id: super::cct::IdentifierType,
    #[serde(rename = "VersionID")]
    pub version_id: super::cct::IdentifierType,
    #[serde(default, rename = "SchemaURI")]
    pub schema_uri: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "DocumentTypeCode")]
    pub document_type_code: Option<super::cct::CodeType>,
}
