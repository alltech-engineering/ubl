#[derive(Debug, Deserialize, Serialize)]
pub struct ItemComparison {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "PriceAmount")]
    pub price_amount: Option<cct::Amount>,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
}
