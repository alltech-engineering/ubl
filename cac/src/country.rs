#[derive(Debug, Deserialize, Serialize)]
pub struct Country {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "IdentificationCode")]
    pub identification_code: Option<cct::Code>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
}
