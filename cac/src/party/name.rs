#[derive(Debug, Deserialize, Serialize)]
pub struct PartyName {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(rename = "Name")]
    pub name: cct::Text,
}
