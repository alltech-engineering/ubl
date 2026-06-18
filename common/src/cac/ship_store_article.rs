#[derive(Debug, Deserialize, Serialize)]
pub struct ShipStoreArticle {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Name")]
    pub name: Option<super::cct::TextType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "OfficialUse")]
    pub official_use: Vec<super::cct::TextType>,
    #[serde(default, rename = "Stowage")]
    pub stowage: Option<Stowage>,
}
