// ReceiptLine — UBL CAC aggregate
// A line in a Receipt Advice document.
use crate::cbc::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReceiptLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
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
    pub reject_action_code: Option<RejectActionCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_discrepancy_code: Option<QuantityDiscrepancyCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oversupply_quantity: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_date: Option<ReceivedDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_time: Option<ReceivedTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing_complaint_code: Option<TimingComplaintCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing_complaint: Option<TimingComplaint>,
    // CAC references
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub order_line_reference: Vec<OrderLineReference>,
    // TODO: cac:DespatchLineReference
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub document_reference: Vec<DocumentReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Item>,
    // TODO: cac:Shipment
}

use super::invoice_line::OrderLineReference;
use super::document_reference::DocumentReference;
use super::item::Item;
