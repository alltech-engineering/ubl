// InvoiceLine — UBL CAC aggregate
// A line in an Invoice document.
use crate::cbc::*;

/// A line in an invoice.
/// UBL element: cac:InvoiceLine
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InvoiceLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<Uuid>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub note: Vec<Note>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoiced_quantity: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_extension_amount: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_inclusive_line_extension_amount: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_point_date: Option<TaxPointDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost_code: Option<AccountingCostCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost: Option<AccountingCost>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_purpose_code: Option<PaymentPurposeCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_of_charge_indicator: Option<FreeOfChargeIndicator>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub invoice_period: Vec<Period>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub order_line_reference: Vec<OrderLineReference>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub billing_reference: Vec<BillingReference>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub document_reference: Vec<DocumentReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Item>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Price>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub sub_invoice_line: Vec<InvoiceLine>,
}

use super::billing_reference::BillingReference;
use super::document_reference::DocumentReference;
use super::period::Period;
use super::item::Item;
use super::price::Price;

/// A reference to an order line.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderLineReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_order_line_id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_status_code: Option<LineStatusCode>,
}
