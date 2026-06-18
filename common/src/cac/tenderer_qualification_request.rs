#[derive(Debug, Deserialize, Serialize)]
pub struct TendererQualificationRequest {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "CompanyLegalFormCode")]
    pub company_legal_form_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "CompanyLegalForm")]
    pub company_legal_form: Vec<super::cct::TextType>,
    #[serde(default, rename = "PersonalSituation")]
    pub personal_situation: Vec<super::cct::TextType>,
    #[serde(default, rename = "OperatingYearsQuantity")]
    pub operating_years_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "EmployeeQuantity")]
    pub employee_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "RequiredBusinessClassificationScheme")]
    pub required_business_classification_scheme: Vec<ClassificationScheme>,
    #[serde(default, rename = "TechnicalEvaluationCriterion")]
    pub technical_evaluation_criterion: Vec<EvaluationCriterion>,
    #[serde(default, rename = "FinancialEvaluationCriterion")]
    pub financial_evaluation_criterion: Vec<EvaluationCriterion>,
    #[serde(default, rename = "SpecificTendererRequirement")]
    pub specific_tenderer_requirement: Vec<TendererRequirement>,
    #[serde(default, rename = "EconomicOperatorRole")]
    pub economic_operator_role: Vec<EconomicOperatorRole>,
}
