#[derive(Debug, Deserialize, Serialize)]
/// A class defining the required criterion for a tenderer to be elligible in a tendering process.
///
/// UBL Dictionary Entry Name: `Evaluation Criterion. Details`
///
/// Generated from XSD type `EvaluationCriterionType`.
pub struct EvaluationCriterion {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// A code that specifies the criterion; it may be financial, technical or organizational.
    #[serde(default, rename = "EvaluationCriterionTypeCode")]
    pub evaluation_criterion_type_code: Option<cct::Code>,
/// A description of the criterion.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// Estimated monetary amount of the threshold for the criterion.
    #[serde(default, rename = "ThresholdAmount")]
    pub threshold_amount: Option<cct::Amount>,
/// Estimated quantity of the threshold for the criterion.
    #[serde(default, rename = "ThresholdQuantity")]
    pub threshold_quantity: Option<cct::Quantity>,
/// A code identifying the expression that will be used to evaluate the criterion.
    #[serde(default, rename = "ExpressionCode")]
    pub expression_code: Option<cct::Code>,
/// The expression that will be used to evaluate the criterion.
    #[serde(default, rename = "Expression")]
    pub expression: Vec<cct::Text>,
/// Describes the period for which the evaluation criterion is valid.
    #[serde(default, rename = "DurationPeriod")]
    pub duration_period: Option<Period>,
/// Describes any evidences that ought to be used to satisfy the criterion.
    #[serde(default, rename = "SuggestedEvidence")]
    pub suggested_evidence: Vec<Evidence>,
}
