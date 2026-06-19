#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line describing a request for a catalogue line.
///
/// UBL Dictionary Entry Name: `Catalogue Request Line. Details`
///
/// Generated from XSD type `CatalogueRequestLineType`.
pub struct CatalogueRequestLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for the requested catalogue line.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// A subdivision of a contract or tender covering the line being requested.
    #[serde(default, rename = "ContractSubdivision")]
    pub contract_subdivision: Option<cct::Text>,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// The period for which the information in the requested catalogue line is valid.
    #[serde(default, rename = "LineValidityPeriod")]
    pub line_validity_period: Option<crate::Period>,
/// Properties of the item in the requested catalogue line that are dependent on location and quantity.
    #[serde(default, rename = "RequiredItemLocationQuantity")]
    pub required_item_location_quantity: Vec<crate::ItemLocationQuantity>,
/// The item associated with the requested catalogue line.
    #[serde(rename = "Item")]
    pub item: crate::Item,
}
