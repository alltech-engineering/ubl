#[derive(Debug, Deserialize, Serialize)]
pub struct SubcontractTerms {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Rate")]
    pub rate: Option<cct::Numeric>,
    #[serde(default, rename = "UnknownPriceIndicator")]
    pub unknown_price_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "Amount")]
    pub amount: Option<cct::Amount>,
    #[serde(default, rename = "SubcontractingConditionsCode")]
    pub subcontracting_conditions_code: Option<cct::Code>,
    #[serde(default, rename = "MaximumPercent")]
    pub maximum_percent: Option<cct::Numeric>,
    #[serde(default, rename = "MinimumPercent")]
    pub minimum_percent: Option<cct::Numeric>,
}
