#[derive(Debug, Deserialize, Serialize)]
pub struct AwardingCriterion {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "AwardingCriterionTypeCode")]
    pub awarding_criterion_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Name")]
    pub name: Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "WeightNumeric")]
    pub weight_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "Weight")]
    pub weight: Vec<super::cct::TextType>,
    #[serde(default, rename = "CalculationExpression")]
    pub calculation_expression: Vec<super::cct::TextType>,
    #[serde(default, rename = "CalculationExpressionCode")]
    pub calculation_expression_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "MinimumAmount")]
    pub minimum_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "MaximumAmount")]
    pub maximum_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "MinimumImprovementBid")]
    pub minimum_improvement_bid: Vec<super::cct::TextType>,
    #[serde(default, rename = "SubordinateAwardingCriterion")]
    pub subordinate_awarding_criterion: Vec<AwardingCriterion>,
}
