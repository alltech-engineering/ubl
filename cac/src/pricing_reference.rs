#[derive(Debug, Deserialize, Serialize)]
pub struct PricingReference {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "OriginalItemLocationQuantity")]
    pub original_item_location_quantity: Option<Box<ItemLocationQuantity>>,
    #[serde(default, rename = "AlternativeConditionPrice")]
    pub alternative_condition_price: Vec<Price>,
}
