#[derive(Debug, Deserialize, Serialize)]
pub struct AwardingCriterion {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "AwardingCriterionTypeCode")]
    pub awarding_criterion_type_code: Option<cct::Code>,
    #[serde(default, rename = "Name")]
    pub name: Vec<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "WeightNumeric")]
    pub weight_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "Weight")]
    pub weight: Vec<cct::Text>,
    #[serde(default, rename = "CalculationExpression")]
    pub calculation_expression: Vec<cct::Text>,
    #[serde(default, rename = "CalculationExpressionCode")]
    pub calculation_expression_code: Option<cct::Code>,
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "MinimumAmount")]
    pub minimum_amount: Option<cct::Amount>,
    #[serde(default, rename = "MaximumAmount")]
    pub maximum_amount: Option<cct::Amount>,
    #[serde(default, rename = "MinimumImprovementBid")]
    pub minimum_improvement_bid: Vec<cct::Text>,
    #[serde(default, rename = "SubordinateAwardingCriterion")]
    pub subordinate_awarding_criterion: Vec<AwardingCriterion>,
}
