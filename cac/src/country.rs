#[derive(Debug, Deserialize, Serialize)]
pub struct Country {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "IdentificationCode")]
    pub identification_code: Option<cct::Code>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
}
