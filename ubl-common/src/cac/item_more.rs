// UBL 2.5 CAC Tier 4: Item instance, lot identification, property groups
//
// Reference: UBL-CommonAggregateComponents-2.5.xsd

use serde::{Deserialize, Serialize};

// ─── ItemInstance ────────────────────────────────────────────────────
// XSD: ItemInstanceType
// A specific, tracked instance of an item (by serial number, lot, etc.)

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemInstance {
    pub product_trace_id: Option<String>,
    pub manufacture_date: Option<String>,
    pub manufacture_time: Option<String>,
    pub best_before_date: Option<String>,
    pub registration_id: Option<String>,
    pub serial_id: Option<String>,
    // CAC: additional_item_property: Vec<ItemProperty>
    // CAC: lot_identification: Option<LotIdentification>
}

// ─── LotIdentification ──────────────────────────────────────────────
// XSD: LotIdentificationType
// A batch or lot identifier for a group of items

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LotIdentification {
    pub lot_number_id: Option<String>,
    pub expiry_date: Option<String>,
    // CAC: additional_item_property: Vec<ItemProperty>
}

// ─── ItemPropertyGroup ───────────────────────────────────────────────
// XSD: ItemPropertyGroupType
// A named group of item properties

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemPropertyGroup {
    pub id: Option<String>,
    pub name: Option<String>,
    pub importance_code: Option<String>,
}

// ─── ItemPropertyRange ───────────────────────────────────────────────
// XSD: ItemPropertyRangeType
// A range of values for an item property

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemPropertyRange {
    pub minimum_value: Option<String>,
    pub maximum_value: Option<String>,
}
