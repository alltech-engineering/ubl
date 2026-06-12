// InvoiceLine — UBL CAC aggregate
// A line in an Invoice document.
use crate::cbc::*;

/// A line in an invoice.
/// UBL element: cac:InvoiceLine
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InvoiceLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
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
    // TODO: cac:DespatchLineReference
    // TODO: cac:ReceiptLineReference
    // TODO: cac:WorkReportLineReference
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub billing_reference: Vec<BillingReference>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub document_reference: Vec<DocumentReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_reference: Option<PricingReference>,
    // TODO: cac:PurchaseReference
    // TODO: cac:OriginatorParty
    // TODO: cac:BeneficiaryParty
    // TODO: cac:CollectedForParty
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub delivery: Vec<Delivery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_terms: Option<PaymentTerms>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub allowance_charge: Vec<AllowanceCharge>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<TaxTotal>,
    // TODO: cac:WithholdingTaxTotal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Item>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Price>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_terms: Option<DeliveryTerms>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub sub_invoice_line: Vec<InvoiceLine>,
    // TODO: cac:ItemPriceExtension
}

use super::billing_reference::BillingReference;
use super::document_reference::DocumentReference;
use super::period::Period;
use super::item::Item;
use super::price::Price;
use super::delivery::Delivery;
use super::payment_terms::PaymentTerms;
use super::allowance_charge::AllowanceCharge;
use super::tax::{PricingReference, TaxTotal};
use super::delivery_terms::DeliveryTerms;

/// A reference to an order line.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderLineReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_id: Option<ID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_order_line_id: Option<ID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_status_code: Option<LineStatusCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_reference: Option<OrderReference>,
}

use super::order_reference::OrderReference;
