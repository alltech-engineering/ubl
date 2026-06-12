// DebitNoteLine — UBL CAC aggregate
// A line in a Debit Note document.
use crate::cbc::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DebitNoteLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub note: Vec<Note>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debited_quantity: Option<Quantity>,
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
    // CAC references
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub invoice_period: Vec<Period>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub order_line_reference: Vec<OrderLineReference>,
    // TODO: cac:DiscrepancyResponse
    // TODO: cac:DespatchLineReference
    // TODO: cac:ReceiptLineReference
    // TODO: cac:WorkReportLineReference
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub billing_reference: Vec<BillingReference>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub document_reference: Vec<DocumentReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_reference: Option<PricingReference>,
    // TODO: cac:OriginatorParty
    // TODO: cac:BeneficiaryParty
    // TODO: cac:CollectedForParty
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub delivery: Vec<Delivery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_terms: Option<PaymentTerms>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tax_total: Vec<TaxTotal>,
    // TODO: cac:WithholdingTaxTotal
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub allowance_charge: Vec<AllowanceCharge>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Item>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Price>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_terms: Option<DeliveryTerms>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub sub_debit_note_line: Vec<DebitNoteLine>,
    // TODO: cac:ItemPriceExtension
}

use super::period::Period;
use super::invoice_line::OrderLineReference;
use super::billing_reference::BillingReference;
use super::document_reference::DocumentReference;
use super::tax::{PricingReference, TaxTotal};
use super::delivery::Delivery;
use super::payment_terms::PaymentTerms;
use super::allowance_charge::AllowanceCharge;
use super::item::Item;
use super::price::Price;
use super::delivery_terms::DeliveryTerms;
