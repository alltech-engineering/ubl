#[derive(Debug, Deserialize, Serialize)]
pub struct LineReference {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "LineID")]
    pub line_id: cct::Identifier,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(default, rename = "LineStatusCode")]
    pub line_status_code: Option<cct::Code>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Option<crate::DocumentReference>,
}
