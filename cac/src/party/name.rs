#[derive(Debug, Deserialize, Serialize)]
pub struct PartyName {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "Name")]
    pub name: cct::Text,
}
