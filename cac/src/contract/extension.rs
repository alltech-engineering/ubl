#[derive(Debug, Deserialize, Serialize)]
pub struct ContractExtension {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "OptionsDescription")]
    pub options_description: Vec<cct::Text>,
    #[serde(default, rename = "MinimumNumberNumeric")]
    pub minimum_number_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "MaximumNumberNumeric")]
    pub maximum_number_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "RenewalsIndicator")]
    pub renewals_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "OptionValidityPeriod")]
    pub option_validity_period: Option<crate::Period>,
    #[serde(default, rename = "Renewal")]
    pub renewal: Vec<crate::Renewal>,
}
