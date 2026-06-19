#[derive(Debug, Deserialize, Serialize)]
pub struct EvaluationCriterion {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "EvaluationCriterionTypeCode")]
    pub evaluation_criterion_type_code: Option<cct::Code>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "ThresholdAmount")]
    pub threshold_amount: Option<cct::Amount>,
    #[serde(default, rename = "ThresholdQuantity")]
    pub threshold_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "ExpressionCode")]
    pub expression_code: Option<cct::Code>,
    #[serde(default, rename = "Expression")]
    pub expression: Vec<cct::Text>,
    #[serde(default, rename = "DurationPeriod")]
    pub duration_period: Option<Period>,
    #[serde(default, rename = "SuggestedEvidence")]
    pub suggested_evidence: Vec<Evidence>,
}
