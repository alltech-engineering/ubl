#[derive(Debug, Deserialize, Serialize)]
pub struct BudgetAccount {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "BudgetYearNumeric")]
    pub budget_year_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "RequiredClassificationScheme")]
    pub required_classification_scheme: Option<ClassificationScheme>,
}
