#[derive(Debug, Deserialize, Serialize)]
pub struct RequestedTenderTotal {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "EstimatedOverallContractAmount")]
    pub estimated_overall_contract_amount: Option<cct::Amount>,
    #[serde(default, rename = "EstimatedOverallFrameworkContractsAmount")]
    pub estimated_overall_framework_contracts_amount: Option<cct::Amount>,
    #[serde(default, rename = "TotalAmount")]
    pub total_amount: Option<cct::Amount>,
    #[serde(default, rename = "TaxIncludedIndicator")]
    pub tax_included_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "MinimumAmount")]
    pub minimum_amount: Option<cct::Amount>,
    #[serde(default, rename = "MaximumAmount")]
    pub maximum_amount: Option<cct::Amount>,
    #[serde(default, rename = "MonetaryScope")]
    pub monetary_scope: Vec<cct::Text>,
    #[serde(default, rename = "AverageSubsequentContractAmount")]
    pub average_subsequent_contract_amount: Option<cct::Amount>,
    #[serde(default, rename = "ApplicableTaxCategory")]
    pub applicable_tax_category: Vec<TaxCategory>,
}
