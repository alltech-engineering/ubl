#[derive(Debug, Deserialize, Serialize)]
pub struct TendererQualificationRequest {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "CompanyLegalFormCode")]
    pub company_legal_form_code: Option<cct::Code>,
    #[serde(default, rename = "CompanyLegalForm")]
    pub company_legal_form: Vec<cct::Text>,
    #[serde(default, rename = "PersonalSituation")]
    pub personal_situation: Vec<cct::Text>,
    #[serde(default, rename = "OperatingYearsQuantity")]
    pub operating_years_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "EmployeeQuantity")]
    pub employee_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "RequiredBusinessClassificationScheme")]
    pub required_business_classification_scheme: Vec<crate::ClassificationScheme>,
    #[serde(default, rename = "TechnicalEvaluationCriterion")]
    pub technical_evaluation_criterion: Vec<crate::EvaluationCriterion>,
    #[serde(default, rename = "FinancialEvaluationCriterion")]
    pub financial_evaluation_criterion: Vec<crate::EvaluationCriterion>,
    #[serde(default, rename = "SpecificTendererRequirement")]
    pub specific_tenderer_requirement: Vec<TendererRequirement>,
    #[serde(default, rename = "EconomicOperatorRole")]
    pub economic_operator_role: Vec<crate::EconomicOperatorRole>,
}
