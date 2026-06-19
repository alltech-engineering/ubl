#[derive(Debug, Deserialize, Serialize)]
/// A class defining budgeted monetary amounts.
///
/// UBL Dictionary Entry Name: `Requested Tender Total. Details`
///
/// Generated from XSD type `RequestedTenderTotalType`.
pub struct RequestedTenderTotal {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The estimated overall monetary amount of a contract.
    #[serde(default, rename = "EstimatedOverallContractAmount")]
    pub estimated_overall_contract_amount: Option<cct::Amount>,
/// The estimated overall monetary amount of subsequent framework contracts.
    #[serde(default, rename = "EstimatedOverallFrameworkContractsAmount")]
    pub estimated_overall_framework_contracts_amount: Option<cct::Amount>,
/// The monetary amount of the total budget including net amount, taxes, and material and instalment
/// costs.
    #[serde(default, rename = "TotalAmount")]
    pub total_amount: Option<cct::Amount>,
/// Indicates whether the amounts are taxes included (true) or not (false).
    #[serde(default, rename = "TaxIncludedIndicator")]
    pub tax_included_indicator: Option<udt::Indicator>,
/// The minimum monetary amount of the budget.
    #[serde(default, rename = "MinimumAmount")]
    pub minimum_amount: Option<cct::Amount>,
/// The maximum monetary amount of the budget.
    #[serde(default, rename = "MaximumAmount")]
    pub maximum_amount: Option<cct::Amount>,
/// A description of the monetary scope of the budget.
    #[serde(default, rename = "MonetaryScope")]
    pub monetary_scope: Vec<cct::Text>,
/// The average monetary amount for the subsequent contracts following this budget amount.
    #[serde(default, rename = "AverageSubsequentContractAmount")]
    pub average_subsequent_contract_amount: Option<cct::Amount>,
/// Describes the categories of taxes that apply to the budget amount.
    #[serde(default, rename = "ApplicableTaxCategory")]
    pub applicable_tax_category: Vec<TaxCategory>,
}
