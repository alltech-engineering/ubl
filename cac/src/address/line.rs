#[derive(Debug, Deserialize, Serialize)]
pub struct AddressLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Line")]
    pub line: Vec<cct::Text>,
}
