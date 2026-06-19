#[derive(Debug, Deserialize, Serialize)]
pub struct NoticeSubType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "SubTypeCode")]
    pub sub_type_code: Option<cct::Code>,
    #[serde(default, rename = "SubTypeDescription")]
    pub sub_type_description: Vec<cct::Text>,
}
