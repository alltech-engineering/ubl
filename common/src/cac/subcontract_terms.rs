#[derive(Debug, Deserialize, Serialize)]
pub struct SubcontractTerms {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Rate")]
    pub rate: Option<super::cct::NumericType>,
    #[serde(default, rename = "UnknownPriceIndicator")]
    pub unknown_price_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "Amount")]
    pub amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "SubcontractingConditionsCode")]
    pub subcontracting_conditions_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "MaximumPercent")]
    pub maximum_percent: Option<super::cct::NumericType>,
    #[serde(default, rename = "MinimumPercent")]
    pub minimum_percent: Option<super::cct::NumericType>,
}
