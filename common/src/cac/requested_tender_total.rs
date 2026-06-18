#[derive(Debug, Deserialize, Serialize)]
pub struct RequestedTenderTotal {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "EstimatedOverallContractAmount")]
    pub estimated_overall_contract_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "EstimatedOverallFrameworkContractsAmount")]
    pub estimated_overall_framework_contracts_amount:
        Option<super::cct::AmountType>,
    #[serde(default, rename = "TotalAmount")]
    pub total_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxIncludedIndicator")]
    pub tax_included_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "MinimumAmount")]
    pub minimum_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "MaximumAmount")]
    pub maximum_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "MonetaryScope")]
    pub monetary_scope: Vec<super::cct::TextType>,
    #[serde(default, rename = "AverageSubsequentContractAmount")]
    pub average_subsequent_contract_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "ApplicableTaxCategory")]
    pub applicable_tax_category: Vec<TaxCategory>,
}
