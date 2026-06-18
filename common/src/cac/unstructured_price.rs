#[derive(Debug, Deserialize, Serialize)]
pub struct UnstructuredPrice {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "PriceAmount")]
    pub price_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxInclusivePriceAmount")]
    pub tax_inclusive_price_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "TimeAmount")]
    pub time_amount: Option<super::cct::TextType>,
}
