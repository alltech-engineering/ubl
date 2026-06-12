// UBL Line aggregates — document line items for various document types.

use serde::{Deserialize, Serialize};
use crate::cbc::*;
use crate::cac::allowance::AllowanceCharge;
use crate::cac::delivery::Delivery;
use crate::cac::item::Item;
use crate::cac::period::Period;
use crate::cac::price::Price;
use crate::cac::tax::TaxTotal;

/// The universal LineItem — used by Order, Catalogue, Quotation, Tender.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineItem {
    pub id: ID,
    pub sales_order_id: Option<SalesOrderID>,
    pub uuid: Option<UUID>,
    pub note: Vec<Note>,
    pub line_status_code: Option<LineStatusCode>,
    pub quantity: Option<Quantity>,
    pub line_extension_amount: Option<LineExtensionAmount>,
    pub total_tax_amount: Option<TotalTaxAmount>,
    pub minimum_quantity: Option<MinimumQuantity>,
    pub maximum_quantity: Option<MaximumQuantity>,
    pub minimum_backorder_quantity: Option<MinimumBackorderQuantity>,
    pub maximum_backorder_quantity: Option<MaximumBackorderQuantity>,
    pub inspection_method_code: Option<InspectionMethodCode>,
    pub partial_delivery_indicator: Option<PartialDeliveryIndicator>,
    pub back_order_allowed_indicator: Option<BackOrderAllowedIndicator>,
    pub accounting_cost_code: Option<AccountingCostCode>,
    pub accounting_cost: Option<AccountingCost>,
    pub delivery: Vec<Delivery>,
    pub delivery_terms: Vec<DeliveryTerms>,
    pub originator_party: Option<Party>,
    pub ordered_shipment: Vec<OrderedShipment>,
    pub pricing_reference: Option<PricingReference>,
    pub allowance_charge: Vec<AllowanceCharge>,
    pub price: Option<Price>,
    pub item: Item,
}

/// InvoiceLine — a line on an invoice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvoiceLine {
    pub id: ID,
    pub uuid: Option<UUID>,
    pub note: Vec<Note>,
    pub invoiced_quantity: Option<InvoicedQuantity>,
    pub line_extension_amount: LineExtensionAmount,
    pub tax_point_date: Option<TaxPointDate>,
    pub accounting_cost_code: Option<AccountingCostCode>,
    pub accounting_cost: Option<AccountingCost>,
    pub payment_purpose_code: Option<PaymentPurposeCode>,
    pub free_of_charge_indicator: Option<Indicator>,
    pub invoice_period: Vec<Period>,
    pub order_line_reference: Vec<OrderLineReference>,
    pub despatch_line_reference: Vec<LineReference>,
    pub receipt_line_reference: Vec<LineReference>,
    pub billing_reference: Vec<BillingReference>,
    pub document_reference: Vec<DocumentReference>,
    pub pricing_reference: Option<PricingReference>,
    pub originator_party: Option<Party>,
    pub delivery: Vec<Delivery>,
    pub payment_terms: Vec<PaymentTerms>,
    pub allowance_charge: Vec<AllowanceCharge>,
    pub tax_total: Vec<TaxTotal>,
    pub withholding_tax_total: Vec<TaxTotal>,
    pub item: Item,
    pub price: Option<Price>,
    pub delivery_terms: Option<DeliveryTerms>,
    pub sub_invoice_line: Vec<Box<InvoiceLine>>,
    pub price_adjustment: Option<Price>,
}

/// CreditNoteLine — a line on a credit note.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreditNoteLine {
    pub id: ID,
    pub uuid: Option<UUID>,
    pub note: Vec<Note>,
    pub credited_quantity: Option<CreditedQuantity>,
    pub line_extension_amount: Option<LineExtensionAmount>,
    pub tax_point_date: Option<TaxPointDate>,
    pub accounting_cost_code: Option<AccountingCostCode>,
    pub accounting_cost: Option<AccountingCost>,
    pub payment_purpose_code: Option<PaymentPurposeCode>,
    pub free_of_charge_indicator: Option<Indicator>,
    pub invoice_period: Vec<Period>,
    pub order_line_reference: Vec<OrderLineReference>,
    pub despatch_line_reference: Vec<LineReference>,
    pub receipt_line_reference: Vec<LineReference>,
    pub billing_reference: Vec<BillingReference>,
    pub document_reference: Vec<DocumentReference>,
    pub pricing_reference: Option<PricingReference>,
    pub originator_party: Option<Party>,
    pub delivery: Vec<Delivery>,
    pub payment_terms: Vec<PaymentTerms>,
    pub allowance_charge: Vec<AllowanceCharge>,
    pub tax_total: Vec<TaxTotal>,
    pub item: Item,
    pub price: Option<Price>,
}

/// DebitNoteLine — a line on a debit note.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DebitNoteLine {
    pub id: ID,
    pub uuid: Option<UUID>,
    pub note: Vec<Note>,
    pub debited_quantity: Option<DebitedQuantity>,
    pub line_extension_amount: Option<LineExtensionAmount>,
    pub tax_point_date: Option<TaxPointDate>,
    pub accounting_cost_code: Option<AccountingCostCode>,
    pub accounting_cost: Option<AccountingCost>,
    pub payment_purpose_code: Option<PaymentPurposeCode>,
    pub invoice_period: Vec<Period>,
    pub order_line_reference: Vec<OrderLineReference>,
    pub despatch_line_reference: Vec<LineReference>,
    pub receipt_line_reference: Vec<LineReference>,
    pub billing_reference: Vec<BillingReference>,
    pub document_reference: Vec<DocumentReference>,
    pub pricing_reference: Option<PricingReference>,
    pub originator_party: Option<Party>,
    pub delivery: Vec<Delivery>,
    pub payment_terms: Vec<PaymentTerms>,
    pub allowance_charge: Vec<AllowanceCharge>,
    pub tax_total: Vec<TaxTotal>,
    pub item: Item,
    pub price: Option<Price>,
}

/// OrderLine — a line on an order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderLine {
    pub id: ID,
    pub sales_order_id: Option<SalesOrderID>,
    pub uuid: Option<UUID>,
    pub note: Vec<Note>,
    pub line_status_code: Option<LineStatusCode>,
    pub quantity: Option<Quantity>,
    pub line_extension_amount: Option<LineExtensionAmount>,
    pub total_tax_amount: Option<TotalTaxAmount>,
    pub minimum_quantity: Option<MinimumQuantity>,
    pub maximum_quantity: Option<MaximumQuantity>,
    pub minimum_backorder_quantity: Option<MinimumBackorderQuantity>,
    pub maximum_backorder_quantity: Option<MaximumBackorderQuantity>,
    pub inspection_method_code: Option<InspectionMethodCode>,
    pub partial_delivery_indicator: Option<PartialDeliveryIndicator>,
    pub back_order_allowed_indicator: Option<BackOrderAllowedIndicator>,
    pub accounting_cost_code: Option<AccountingCostCode>,
    pub accounting_cost: Option<AccountingCost>,
    pub delivery: Vec<Delivery>,
    pub delivery_terms: Vec<DeliveryTerms>,
    pub originator_party: Option<Party>,
    pub ordered_shipment: Vec<OrderedShipment>,
    pub pricing_reference: Option<PricingReference>,
    pub allowance_charge: Vec<AllowanceCharge>,
    pub price: Option<Price>,
    pub item: Item,
    pub line_reference: Vec<LineReference>,
    pub document_reference: Vec<DocumentReference>,
}

/// DespatchLine — a line on a despatch advice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DespatchLine {
    pub id: ID,
    pub uuid: Option<UUID>,
    pub note: Vec<Note>,
    pub line_status_code: Option<LineStatusCode>,
    pub delivered_quantity: Option<DeliveredQuantity>,
    pub outstanding_quantity: Option<Quantity>,
    pub outstanding_reason: Vec<OutstandingReason>,
    pub order_line_reference: Vec<OrderLineReference>,
    pub document_reference: Vec<DocumentReference>,
    pub item: Item,
    pub shipment: Vec<Shipment>,
}

/// ReceiptLine — a line on a receipt advice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReceiptLine {
    pub id: ID,
    pub uuid: Option<UUID>,
    pub note: Vec<Note>,
    pub received_quantity: Option<ReceivedQuantity>,
    pub rejected_quantity: Option<RejectedQuantity>,
    pub reject_reason_code: Option<RejectReasonCode>,
    pub reject_reason: Vec<RejectionNote>,
    pub order_line_reference: Vec<OrderLineReference>,
    pub despatch_line_reference: Vec<LineReference>,
    pub document_reference: Vec<DocumentReference>,
    pub item: Item,
}

// --- Support types ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineReference {
    pub line_id: LineID,
    pub line_status_code: Option<LineStatusCode>,
    pub document_reference: Option<DocumentReference>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderLineReference {
    pub line_id: LineID,
    pub sales_order_line_id: Option<SalesOrderLineID>,
    pub uuid: Option<UUID>,
    pub line_status_code: Option<LineStatusCode>,
    pub order_reference: Option<OrderReference>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderedShipment {
    pub shipment: Shipment,
    pub package: Vec<Package>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Package {
    pub id: Option<ID>,
    pub quantity: Option<Quantity>,
    pub returnable_material_indicator: Option<ReturnableMaterialIndicator>,
    pub package_level_code: Option<Code>,
    pub packaging_type_code: Option<PackagingTypeCode>,
    pub goods_item: Vec<GoodsItem>,
}

/// InstructionForReturnsLine — a line on an InstructionForReturns.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstructionForReturnsLine {
    pub id: ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub note: Vec<Note>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Quantity>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub order_line_reference: Vec<OrderLineReference>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub document_reference: Vec<DocumentReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Item>,
}

// Cross-module imports
use crate::cac::delivery::{DeliveryTerms, GoodsItem, Shipment};
use crate::cac::document::DocumentReference;
use crate::cac::party::Party;
use crate::cac::payment::PaymentTerms;

// Additional imports
use crate::cac::order_reference::{BillingReference, OrderReference};
use crate::cac::tax::PricingReference;
