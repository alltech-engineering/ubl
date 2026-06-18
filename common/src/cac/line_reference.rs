#[derive(Debug, Deserialize, Serialize)]
pub struct LineReference {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "LineID")]
    pub line_id: super::cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "LineStatusCode")]
    pub line_status_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Option<DocumentReference>,
}
