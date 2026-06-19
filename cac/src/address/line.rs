#[derive(Debug, Deserialize, Serialize)]
pub struct AddressLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "Line")]
    pub line: Vec<cct::Text>,
}
