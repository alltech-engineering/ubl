use serde::{Deserialize, Serialize};

include!("account.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct BudgetAccountLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "TotalAmount")]
    pub total_amount: Option<cct::Amount>,
    #[serde(default, rename = "BudgetAccount")]
    pub budget_account: Vec<BudgetAccount>,
}
