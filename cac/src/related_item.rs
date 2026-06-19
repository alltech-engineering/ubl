#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the relationship to an item different from the item associated with the item
/// line in which RelatedItem is used.
///
/// UBL Dictionary Entry Name: `Related Item. Details`
///
/// Generated from XSD type `RelatedItemType`.
pub struct RelatedItem {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for the related item.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The quantity that applies to the relationship.
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
/// Text describing the relationship.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}
