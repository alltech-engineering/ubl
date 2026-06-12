// Item — UBL CAC aggregate (Tier 1 stub)
// Placeholder for the full Tier 1 Item aggregate.
use crate::cbc::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commodity_code: Option<CommodityCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_classification_code: Option<ItemClassificationCode>,
}
