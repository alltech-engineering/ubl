#[derive(Debug, Deserialize, Serialize)]
pub struct NoticeSubType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "SubTypeCode")]
    pub sub_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "SubTypeDescription")]
    pub sub_type_description: Vec<super::cct::TextType>,
}
