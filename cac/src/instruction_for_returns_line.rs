#[derive(Debug, Deserialize, Serialize)]
pub struct InstructionForReturnsLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(rename = "Quantity")]
    pub quantity: cct::Quantity,
    #[serde(default, rename = "ManufacturerParty")]
    pub manufacturer_party: Option<Party>,
    #[serde(rename = "Item")]
    pub item: Item,
}
