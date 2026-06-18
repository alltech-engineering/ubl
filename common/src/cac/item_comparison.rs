#[derive(Debug, Deserialize, Serialize)]
pub struct ItemComparison {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "PriceAmount")]
    pub price_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<super::cct::QuantityType>,
}
