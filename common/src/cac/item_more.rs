// UBL 2.5 CAC Tier 4: Item instance, lot identification, property groups
//
// Reference: UBL-CommonAggregateComponents-2.5.xsd

use serde::{Deserialize, Serialize};

// ─── ItemInstance ────────────────────────────────────────────────────
// XSD: ItemInstanceType
// A specific, tracked instance of an item (by serial number, lot, etc.)

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemInstance {
    #[serde(default)]
    pub product_trace_id: Option<String>,
    #[serde(default)]
    pub manufacture_date: Option<String>,
    #[serde(default)]
    pub manufacture_time: Option<String>,
    #[serde(default)]
    pub best_before_date: Option<String>,
    #[serde(default)]
    pub registration_id: Option<String>,
    #[serde(default)]
    pub serial_id: Option<String>,
    // CAC: additional_item_property: Vec<ItemProperty>
    // CAC: lot_identification: Option<LotIdentification>
}

// ─── LotIdentification ──────────────────────────────────────────────
// XSD: LotIdentificationType
// A batch or lot identifier for a group of items

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LotIdentification {
    #[serde(default)]
    pub lot_number_id: Option<String>,
    #[serde(default)]
    pub expiry_date: Option<String>,
    // CAC: additional_item_property: Vec<ItemProperty>
}

// ─── ItemPropertyGroup ───────────────────────────────────────────────
// XSD: ItemPropertyGroupType
// A named group of item properties

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemPropertyGroup {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub importance_code: Option<String>,
}

// ─── ItemPropertyRange ───────────────────────────────────────────────
// XSD: ItemPropertyRangeType
// A range of values for an item property

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemPropertyRange {
    #[serde(default)]
    pub minimum_value: Option<String>,
    #[serde(default)]
    pub maximum_value: Option<String>,
}
