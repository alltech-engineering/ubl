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
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub despatch_line_reference: Vec<DespatchLineReference>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub receipt_line_reference: Vec<ReceiptLineReference>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub work_report_line_reference: Vec<WorkReportLineReference>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub billing_reference: Vec<BillingReference>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub document_reference: Vec<DocumentReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_reference: Option<PricingReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_reference: Option<PurchaseReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator_party: Option<Party>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_party: Option<Party>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collected_for_party: Option<Party>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub delivery: Vec<Delivery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_terms: Option<PaymentTerms>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub allowance_charge: Vec<AllowanceCharge>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<TaxTotal>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub withholding_tax_total: Vec<TaxTotal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Item>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Price>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_terms: Option<DeliveryTerms>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub sub_invoice_line: Vec<InvoiceLine>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub item_price_extension: Vec<ItemPriceExtension>,
}
use super::allowance_charge::AllowanceCharge;
use super::billing_reference::BillingReference;
use super::delivery::Delivery;
use super::delivery_terms::DeliveryTerms;
use super::document_reference::DocumentReference;
use super::item::Item;
use super::item_price_extension::ItemPriceExtension;
use super::line_reference::{DespatchLineReference, ReceiptLineReference, WorkReportLineReference};
use super::party::Party;
use super::payment_terms::PaymentTerms;
use super::period::Period;
use super::price::Price;
use super::purchase_reference::PurchaseReference;
use super::tax::{PricingReference, TaxTotal};

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
