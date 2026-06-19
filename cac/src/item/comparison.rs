#[derive(Debug, Deserialize, Serialize)]
/// A class to provide information about price and quantity of an item for use in price comparisons
/// based on price, quantity, or measurements.
///
/// UBL Dictionary Entry Name: `Item Comparison. Details`
///
/// Generated from XSD type `ItemComparisonType`.
pub struct ItemComparison {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The price for the Item Comparison
    #[serde(default, rename = "PriceAmount")]
    pub price_amount: Option<cct::Amount>,
/// The quantity for which this comparison is valid.
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
}
