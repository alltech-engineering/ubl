#[derive(Debug, Deserialize, Serialize)]
/// A class to define a criterion from the contracting party that will be taken into account when
/// awarding a contract. An awarding criterion can be objective, when it can be evaluated following a
/// formula, or subjective, when human analysis is required.
///
/// UBL Dictionary Entry Name: `Awarding Criterion. Details`
///
/// Generated from XSD type `AwardingCriterionType`.
pub struct AwardingCriterion {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// Identifies a specific awarding criterion.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A code used to define this awarding criterion.
    #[serde(default, rename = "AwardingCriterionTypeCode")]
    pub awarding_criterion_type_code: Option<cct::Code>,
/// The name of this awarding criterion.
    #[serde(default, rename = "Name")]
    pub name: Vec<cct::Text>,
/// A description of the awarding criterion.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// A number defining the comparative weighting assigned to this awarding criterion, to enable formulaic
/// evaluation.
    #[serde(default, rename = "WeightNumeric")]
    pub weight_numeric: Option<cct::Numeric>,
/// A description of the comparative weighting for this awarding criterion.
    #[serde(default, rename = "Weight")]
    pub weight: Vec<cct::Text>,
/// The mathematical expression that will be used to evaluate this criterion.
    #[serde(default, rename = "CalculationExpression")]
    pub calculation_expression: Vec<cct::Text>,
/// A code identifying the mathematical expression that will be used to evaluate this criterion.
    #[serde(default, rename = "CalculationExpressionCode")]
    pub calculation_expression_code: Option<cct::Code>,
/// The minimum quantity for an awarding criterion.
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: Option<cct::Quantity>,
/// The maximum quantity for an awarding criterion.
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: Option<cct::Quantity>,
/// The minimum monetary amount for an awarding criterion.
    #[serde(default, rename = "MinimumAmount")]
    pub minimum_amount: Option<cct::Amount>,
/// The maximum monetary amount for an awarding criterion.
    #[serde(default, rename = "MaximumAmount")]
    pub maximum_amount: Option<cct::Amount>,
/// Describes the minimum improvement bid for this awarding criterion when used in an auction.
    #[serde(default, rename = "MinimumImprovementBid")]
    pub minimum_improvement_bid: Vec<cct::Text>,
/// Defines any subsidiary awarding criterion.
    #[serde(default, rename = "SubordinateAwardingCriterion")]
    pub subordinate_awarding_criterion: Vec<AwardingCriterion>,
}
