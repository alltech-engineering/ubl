#[derive(Debug, Deserialize, Serialize)]
pub struct InstructionForReturnsLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(rename = "Quantity")]
    pub quantity: super::cct::QuantityType,
    #[serde(default, rename = "ManufacturerParty")]
    pub manufacturer_party: Option<Party>,
    #[serde(rename = "Item")]
    pub item: Item,
}
