#[derive(Debug, Deserialize, Serialize)]
pub struct PricingReference {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "OriginalItemLocationQuantity")]
    pub original_item_location_quantity:
        Option<Box<ItemLocationQuantity>>,
    #[serde(default, rename = "AlternativeConditionPrice")]
    pub alternative_condition_price: Vec<Price>,
}
