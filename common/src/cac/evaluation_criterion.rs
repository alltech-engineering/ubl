#[derive(Debug, Deserialize, Serialize)]
pub struct EvaluationCriterion {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "EvaluationCriterionTypeCode")]
    pub evaluation_criterion_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "ThresholdAmount")]
    pub threshold_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "ThresholdQuantity")]
    pub threshold_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "ExpressionCode")]
    pub expression_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Expression")]
    pub expression: Vec<super::cct::TextType>,
    #[serde(default, rename = "DurationPeriod")]
    pub duration_period: Option<Period>,
    #[serde(default, rename = "SuggestedEvidence")]
    pub suggested_evidence: Vec<Evidence>,
}
