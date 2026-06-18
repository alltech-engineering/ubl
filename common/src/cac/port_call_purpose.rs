#[derive(Debug, Deserialize, Serialize)]
pub struct PortCallPurpose {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "PurposeTypeCode")]
    pub purpose_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "PurposeType")]
    pub purpose_type: Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
}
