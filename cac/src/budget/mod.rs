use serde::{Deserialize, Serialize};

include!("account.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to define a budget account line.
///
/// UBL Dictionary Entry Name: `Budget Account Line. Details`
///
/// Generated from XSD type `BudgetAccountLineType`.
pub struct BudgetAccountLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this budget account line.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The total monetary amount for this budget account line.
    #[serde(default, rename = "TotalAmount")]
    pub total_amount: Option<cct::Amount>,
/// An account covering this budget account line.
    #[serde(default, rename = "BudgetAccount")]
    pub budget_account: Vec<BudgetAccount>,
}
