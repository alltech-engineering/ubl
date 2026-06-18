#[derive(Debug, Deserialize, Serialize)]
pub struct ContractExtension {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "OptionsDescription")]
    pub options_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "MinimumNumberNumeric")]
    pub minimum_number_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "MaximumNumberNumeric")]
    pub maximum_number_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "RenewalsIndicator")]
    pub renewals_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "OptionValidityPeriod")]
    pub option_validity_period: Option<Period>,
    #[serde(default, rename = "Renewal")]
    pub renewal: Vec<Renewal>,
}
