#[derive(Debug, Deserialize, Serialize)]
pub struct PriceExtension {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "Amount")]
    pub amount: super::cct::AmountType,
    #[serde(default, rename = "TaxInclusiveAmount")]
    pub tax_inclusive_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<TaxTotal>,
}
