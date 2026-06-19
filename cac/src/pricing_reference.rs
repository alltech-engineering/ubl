#[derive(Debug, Deserialize, Serialize)]
/// A reference to the basis for pricing. This may be based on a catalogue or a quoted amount from a
/// price list and include some alternative pricing conditions.
///
/// UBL Dictionary Entry Name: `Pricing Reference. Details`
///
/// Generated from XSD type `PricingReferenceType`.
pub struct PricingReference {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An original set of location-specific properties (e.g., price and quantity) associated with this
/// item.
    #[serde(default, rename = "OriginalItemLocationQuantity")]
    pub original_item_location_quantity: Option<Box<ItemLocationQuantity>>,
/// The price expressed in terms other than the actual price, e.g., the list price v. the contracted
/// price, or the price in bags v. the price in kilos, or the list price in bags v. the contracted price
/// in kilos.
    #[serde(default, rename = "AlternativeConditionPrice")]
    pub alternative_condition_price: Vec<Price>,
}
