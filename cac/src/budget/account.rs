#[derive(Debug, Deserialize, Serialize)]
pub struct BudgetAccount {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "BudgetYearNumeric")]
    pub budget_year_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "RequiredClassificationScheme")]
    pub required_classification_scheme: Option<crate::ClassificationScheme>,
}
