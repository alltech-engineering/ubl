#[derive(Debug, Deserialize, Serialize)]
/// The evaluation that the Contracting Authority party requests to fulfill to the tenderers.
///
/// UBL Dictionary Entry Name: `Tenderer Qualification Request. Details`
///
/// Generated from XSD type `TendererQualificationRequestType`.
pub struct TendererQualificationRequest {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The legal status requested for potential tenderers, expressed as a code.
    #[serde(default, rename = "CompanyLegalFormCode")]
    pub company_legal_form_code: Option<cct::Code>,
/// The legal status requested for potential tenderers, expressed as text
    #[serde(default, rename = "CompanyLegalForm")]
    pub company_legal_form: Vec<cct::Text>,
/// Text describing the personal situation of the economic operators in this tendering process.
    #[serde(default, rename = "PersonalSituation")]
    pub personal_situation: Vec<cct::Text>,
/// Textual description of the legal form required for potential tenderers.
    #[serde(default, rename = "OperatingYearsQuantity")]
    pub operating_years_quantity: Option<cct::Quantity>,
/// Textual description of the legal form required for potential tenderers.
    #[serde(default, rename = "EmployeeQuantity")]
    pub employee_quantity: Option<cct::Quantity>,
/// Text describing the evaluation requirements for this tenderer.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// A classification scheme for the business profile.
    #[serde(default, rename = "RequiredBusinessClassificationScheme")]
    pub required_business_classification_scheme: Vec<crate::ClassificationScheme>,
/// A technical evaluation criterion required for an economic operator in a tendering process.
    #[serde(default, rename = "TechnicalEvaluationCriterion")]
    pub technical_evaluation_criterion: Vec<crate::EvaluationCriterion>,
/// A financial evaluation criterion required for an economic operator in a tendering process.
    #[serde(default, rename = "FinancialEvaluationCriterion")]
    pub financial_evaluation_criterion: Vec<crate::EvaluationCriterion>,
/// A requirement to be met by a tenderer.
    #[serde(default, rename = "SpecificTendererRequirement")]
    pub specific_tenderer_requirement: Vec<TendererRequirement>,
/// A class to describe the tenderer contracting role.
    #[serde(default, rename = "EconomicOperatorRole")]
    pub economic_operator_role: Vec<crate::EconomicOperatorRole>,
}
