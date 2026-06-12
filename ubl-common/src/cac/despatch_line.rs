// DespatchLine — UBL CAC aggregate
// A line in a Despatch Advice document.
use crate::cbc::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DespatchLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub note: Vec<Note>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_status_code: Option<LineStatusCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered_quantity: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backorder_quantity: Option<Quantity>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub backorder_reason: Vec<BackorderReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outstanding_quantity: Option<Quantity>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub outstanding_reason: Vec<OutstandingReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oversupply_quantity: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost_code: Option<AccountingCostCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost: Option<AccountingCost>,
    // CAC references
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub order_line_reference: Vec<OrderLineReference>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub document_reference: Vec<DocumentReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Item>,
    // TODO: cac:Shipment — not yet wired
    // TODO: cac:SubDespatchLine — recursive
}

use super::invoice_line::OrderLineReference;
use super::document_reference::DocumentReference;
use super::item::Item;
