#[derive(Debug, Deserialize, Serialize)]
pub struct SalesItem {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "Quantity")]
    pub quantity: super::cct::QuantityType,
    #[serde(default, rename = "ActivityProperty")]
    pub activity_property: Vec<crate::cac::activity::Property>,
    #[serde(default, rename = "TaxExclusivePrice")]
    pub tax_exclusive_price: Vec<Price>,
    #[serde(default, rename = "TaxInclusivePrice")]
    pub tax_inclusive_price: Vec<Price>,
    #[serde(rename = "Item")]
    pub item: Item,
}
