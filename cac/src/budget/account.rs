#[derive(Debug, Deserialize, Serialize)]
/// A class to define a budget account.
///
/// UBL Dictionary Entry Name: `Budget Account. Details`
///
/// Generated from XSD type `BudgetAccountType`.
pub struct BudgetAccount {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for the budget account, typically an internal accounting reference.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The number of the year for this budget account, e.g. 2012
    #[serde(default, rename = "BudgetYearNumeric")]
    pub budget_year_numeric: Option<cct::Numeric>,
/// A classification scheme required for this budget account.
    #[serde(default, rename = "RequiredClassificationScheme")]
    pub required_classification_scheme: Option<crate::ClassificationScheme>,
}
