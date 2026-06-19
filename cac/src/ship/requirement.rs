#[derive(Debug, Deserialize, Serialize)]
pub struct ShipRequirement {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}
