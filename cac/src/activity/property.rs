#[derive(Debug, Deserialize, Serialize)]
pub struct Property {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "Name")]
    pub name: cct::Text,
    #[serde(rename = "Value")]
    pub value: cct::Text,
}
