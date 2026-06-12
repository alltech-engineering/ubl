// ReceiptLine — UBL CAC aggregate
// A line in a Receipt Advice document.
use crate::cbc::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReceiptLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<Uuid>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub note: Vec<Note>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_quantity: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_quantity: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortage_action_code: Option<ShortageActionCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_quantity: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason_code: Option<RejectReasonCode>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub reject_reason: Vec<RejectReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Item>,
}
use super::item::Item;
