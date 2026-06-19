#[derive(Debug, Deserialize, Serialize)]
pub struct PriceList {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "StatusCode")]
    pub status_code: Option<cct::Code>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Vec<crate::Period>,
    #[serde(default, rename = "PreviousPriceList")]
    pub previous_price_list: Option<Box<PriceList>>,
}
