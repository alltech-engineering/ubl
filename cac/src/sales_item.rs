#[derive(Debug, Deserialize, Serialize)]
/// A class to describe information related to an item in a sales context
///
/// UBL Dictionary Entry Name: `Sales Item. Details`
///
/// Generated from XSD type `SalesItemType`.
pub struct SalesItem {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The quantity the given information are related to
    #[serde(rename = "Quantity")]
    pub quantity: cct::Quantity,
/// A class to describe the activity (for example "sales", "movement", ...) related to the item.
    #[serde(default, rename = "ActivityProperty")]
    pub activity_property: Vec<activity::Property>,
/// A price for this sales item, exclusive of tax.
    #[serde(default, rename = "TaxExclusivePrice")]
    pub tax_exclusive_price: Vec<Price>,
/// A price for this sales item, including tax.
    #[serde(default, rename = "TaxInclusivePrice")]
    pub tax_inclusive_price: Vec<Price>,
/// The sales item itself.
    #[serde(rename = "Item")]
    pub item: Item,
}
