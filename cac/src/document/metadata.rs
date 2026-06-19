#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentMetadata {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(rename = "FormatID")]
    pub format_id: cct::Identifier,
    #[serde(rename = "VersionID")]
    pub version_id: cct::Identifier,
    #[serde(default, rename = "SchemaURI")]
    pub schema_uri: Option<cct::Identifier>,
    #[serde(default, rename = "DocumentTypeCode")]
    pub document_type_code: Option<cct::Code>,
}
