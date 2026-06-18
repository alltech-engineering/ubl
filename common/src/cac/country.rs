#[derive(Debug, Deserialize, Serialize)]
pub struct Country {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "IdentificationCode")]
    pub identification_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Name")]
    pub name: Option<super::cct::TextType>,
}
