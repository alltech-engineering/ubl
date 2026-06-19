#[derive(Debug, Deserialize, Serialize)]
pub struct FuelProperty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "TypeID")]
    pub type_id: cct::Identifier,
    #[serde(rename = "Value")]
    pub value: cct::Text,
}
