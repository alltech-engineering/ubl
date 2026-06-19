#[derive(Debug, Deserialize, Serialize)]
pub struct UnstructuredPrice {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "PriceAmount")]
    pub price_amount: Option<cct::Amount>,
    #[serde(default, rename = "TaxInclusivePriceAmount")]
    pub tax_inclusive_price_amount: Option<cct::Amount>,
    #[serde(default, rename = "TimeAmount")]
    pub time_amount: Option<cct::Text>,
}
