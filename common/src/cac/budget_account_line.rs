#[derive(Debug, Deserialize, Serialize)]
pub struct BudgetAccountLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "TotalAmount")]
    pub total_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "BudgetAccount")]
    pub budget_account: Vec<BudgetAccount>,
}
