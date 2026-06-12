// OrderLine — UBL CAC aggregate
// A line in an Order document.
use crate::cbc::*;

/// A line in an order.
/// UBL element: cac:OrderLine
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitution_status_code: Option<SubstitutionStatusCode>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub note: Vec<Note>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_item: Option<LineItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_proposed_substitute_line_item: Option<LineItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_substituted_line_item: Option<LineItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_proposed_substitute_line_item: Option<LineItem>,
    // TODO: cac:CatalogueLineReference — not yet implemented
    // TODO: cac:QuotationLineReference — not yet implemented
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub order_line_reference: Vec<OrderLineReference>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub document_reference: Vec<DocumentReference>,
}

use super::line_item::LineItem;
use super::invoice_line::OrderLineReference;
use super::document_reference::DocumentReference;
