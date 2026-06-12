// Price — UBL CAC aggregate (Tier 1 stub)
// Placeholder for the full Tier 1 Price aggregate.
use crate::cbc::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Price {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_amount: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_quantity: Option<Quantity>,
}
