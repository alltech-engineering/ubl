#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line in an Instruction for Returns.
///
/// UBL Dictionary Entry Name: `Instruction For Returns Line. Details`
///
/// Generated from XSD type `InstructionForReturnsLineType`.
pub struct InstructionForReturnsLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this instruction for returns line.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// The quantity of goods being returned.
    #[serde(rename = "Quantity")]
    pub quantity: cct::Quantity,
/// The Party who manufactures the Goods being returned.
    #[serde(default, rename = "ManufacturerParty")]
    pub manufacturer_party: Option<Party>,
/// A description of the item being returned.
    #[serde(rename = "Item")]
    pub item: Item,
}
