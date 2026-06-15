// UBL Line aggregates — document line items for various document types.

use crate::cac::allowance::AllowanceCharge;
use crate::cac::delivery::Delivery;
use crate::cac::item::Item;
use crate::cac::period::Period;
use crate::cac::price::Price;
use crate::cac::tax::TaxTotal;
use crate::cbc::*;
use serde::{Deserialize, Serialize};

/// InvoiceLine — a line on an invoice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvoiceLine {
    pub id: ID,
    #[serde(default)]
    pub uuid: Option<UUID>,
    #[serde(default)]
    pub note: Vec<Note>,
    #[serde(default)]
    pub invoiced_quantity: Option<InvoicedQuantity>,
    pub line_extension_amount: LineExtensionAmount,
    #[serde(default)]
    pub tax_inclusive_line_extension_amount: Option<TaxInclusiveLineExtensionAmount>,
    #[serde(default)]
    pub tax_point_date: Option<TaxPointDate>,
    #[serde(default)]
    pub accounting_cost_code: Option<AccountingCostCode>,
    #[serde(default)]
    pub accounting_cost: Option<AccountingCost>,
    #[serde(default)]
    pub payment_purpose_code: Option<PaymentPurposeCode>,
    #[serde(default)]
    pub free_of_charge_indicator: Option<FreeOfChargeIndicator>,
    #[serde(default)]
    pub invoice_period: Vec<Period>,
    #[serde(default)]
    pub order_line_reference: Vec<OrderLineReference>,
    #[serde(default)]
    pub despatch_line_reference: Vec<LineReference>,
    #[serde(default)]
    pub receipt_line_reference: Vec<LineReference>,
    #[serde(default)]
    pub billing_reference: Vec<BillingReference>,
    #[serde(default)]
    pub document_reference: Vec<DocumentReference>,
    #[serde(default)]
    pub pricing_reference: Option<PricingReference>,
    #[serde(default)]
    pub originator_party: Option<Party>,
    #[serde(default)]
    pub delivery: Vec<Delivery>,
    #[serde(default)]
    pub payment_terms: Vec<PaymentTerms>,
    #[serde(default)]
    pub allowance_charge: Vec<AllowanceCharge>,
    #[serde(default)]
    pub tax_total: Vec<TaxTotal>,
    #[serde(default)]
    pub withholding_tax_total: Vec<TaxTotal>,
    pub item: Item,
    #[serde(default)]
    pub price: Option<Price>,
    #[serde(default)]
    pub delivery_terms: Option<DeliveryTerms>,
    #[serde(default)]
    pub sub_invoice_line: Vec<Box<InvoiceLine>>,
    #[serde(default)]
    pub price_adjustment: Option<Price>,
}

/// CreditNoteLine — a line on a credit note.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreditNoteLine {
    pub id: ID,
    #[serde(default)]
    pub uuid: Option<UUID>,
    #[serde(default)]
    pub note: Vec<Note>,
    #[serde(default)]
    pub credited_quantity: Option<CreditedQuantity>,
    #[serde(default)]
    pub line_extension_amount: Option<LineExtensionAmount>,
    #[serde(default)]
    pub tax_inclusive_line_extension_amount: Option<TaxInclusiveLineExtensionAmount>,
    #[serde(default)]
    pub tax_point_date: Option<TaxPointDate>,
    #[serde(default)]
    pub accounting_cost_code: Option<AccountingCostCode>,
    #[serde(default)]
    pub accounting_cost: Option<AccountingCost>,
    #[serde(default)]
    pub payment_purpose_code: Option<PaymentPurposeCode>,
    #[serde(default)]
    pub free_of_charge_indicator: Option<FreeOfChargeIndicator>,
    #[serde(default)]
    pub invoice_period: Vec<Period>,
    #[serde(default)]
    pub order_line_reference: Vec<OrderLineReference>,
    #[serde(default)]
    pub despatch_line_reference: Vec<LineReference>,
    #[serde(default)]
    pub receipt_line_reference: Vec<LineReference>,
    #[serde(default)]
    pub billing_reference: Vec<BillingReference>,
    #[serde(default)]
    pub document_reference: Vec<DocumentReference>,
    #[serde(default)]
    pub pricing_reference: Option<PricingReference>,
    #[serde(default)]
    pub originator_party: Option<Party>,
    #[serde(default)]
    pub delivery: Vec<Delivery>,
    #[serde(default)]
    pub payment_terms: Vec<PaymentTerms>,
    #[serde(default)]
    pub allowance_charge: Vec<AllowanceCharge>,
    #[serde(default)]
    pub tax_total: Vec<TaxTotal>,
    pub item: Item,
    #[serde(default)]
    pub price: Option<Price>,
}

/// DebitNoteLine — a line on a debit note.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DebitNoteLine {
    pub id: ID,
    #[serde(default)]
    pub uuid: Option<UUID>,
    #[serde(default)]
    pub note: Vec<Note>,
    #[serde(default)]
    pub debited_quantity: Option<DebitedQuantity>,
    #[serde(default)]
    pub line_extension_amount: Option<LineExtensionAmount>,
    #[serde(default)]
    pub tax_inclusive_line_extension_amount: Option<TaxInclusiveLineExtensionAmount>,
    #[serde(default)]
    pub tax_point_date: Option<TaxPointDate>,
    #[serde(default)]
    pub accounting_cost_code: Option<AccountingCostCode>,
    #[serde(default)]
    pub accounting_cost: Option<AccountingCost>,
    #[serde(default)]
    pub payment_purpose_code: Option<PaymentPurposeCode>,
    #[serde(default)]
    pub free_of_charge_indicator: Option<FreeOfChargeIndicator>,
    #[serde(default)]
    pub invoice_period: Vec<Period>,
    #[serde(default)]
    pub order_line_reference: Vec<OrderLineReference>,
    #[serde(default)]
    pub despatch_line_reference: Vec<LineReference>,
    #[serde(default)]
    pub receipt_line_reference: Vec<LineReference>,
    #[serde(default)]
    pub billing_reference: Vec<BillingReference>,
    #[serde(default)]
    pub document_reference: Vec<DocumentReference>,
    #[serde(default)]
    pub pricing_reference: Option<PricingReference>,
    #[serde(default)]
    pub originator_party: Option<Party>,
    #[serde(default)]
    pub delivery: Vec<Delivery>,
    #[serde(default)]
    pub payment_terms: Vec<PaymentTerms>,
    #[serde(default)]
    pub allowance_charge: Vec<AllowanceCharge>,
    #[serde(default)]
    pub tax_total: Vec<TaxTotal>,
    pub item: Item,
    #[serde(default)]
    pub price: Option<Price>,
}

/// DespatchLine — a line on a despatch advice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DespatchLine {
    pub id: ID,
    #[serde(default)]
    pub uuid: Option<UUID>,
    #[serde(default)]
    pub note: Vec<Note>,
    #[serde(default)]
    pub line_status_code: Option<LineStatusCode>,
    #[serde(default)]
    pub delivered_quantity: Option<DeliveredQuantity>,
    #[serde(default)]
    pub outstanding_quantity: Option<Quantity>,
    #[serde(default)]
    pub outstanding_reason: Vec<OutstandingReason>,
    #[serde(default)]
    pub order_line_reference: Vec<OrderLineReference>,
    #[serde(default)]
    pub document_reference: Vec<DocumentReference>,
    pub item: Item,
    #[serde(default)]
    pub shipment: Vec<Shipment>,
}

/// ReceiptLine — a line on a receipt advice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReceiptLine {
    pub id: ID,
    #[serde(default)]
    pub uuid: Option<UUID>,
    #[serde(default)]
    pub note: Vec<Note>,
    #[serde(default)]
    pub received_quantity: Option<ReceivedQuantity>,
    #[serde(default)]
    pub rejected_quantity: Option<RejectedQuantity>,
    #[serde(default)]
    pub reject_reason_code: Option<RejectReasonCode>,
    #[serde(default)]
    pub reject_reason: Vec<RejectionNote>,
    #[serde(default)]
    pub order_line_reference: Vec<OrderLineReference>,
    #[serde(default)]
    pub despatch_line_reference: Vec<LineReference>,
    #[serde(default)]
    pub document_reference: Vec<DocumentReference>,
    pub item: Item,
}

// --- Support types ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineReference {
    pub line_id: LineID,
    #[serde(default)]
    pub line_status_code: Option<LineStatusCode>,
    #[serde(default)]
    pub document_reference: Option<DocumentReference>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderLineReference {
    pub line_id: LineID,
    #[serde(default)]
    pub sales_order_line_id: Option<SalesOrderLineID>,
    #[serde(default)]
    pub uuid: Option<UUID>,
    #[serde(default)]
    pub line_status_code: Option<LineStatusCode>,
    #[serde(default)]
    pub order_reference: Option<OrderReference>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderedShipment {
    pub shipment: Shipment,
    #[serde(default)]
    pub package: Vec<Package>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Package {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub quantity: Option<Quantity>,
    #[serde(default)]
    pub returnable_material_indicator: Option<ReturnableMaterialIndicator>,
    #[serde(default)]
    pub package_level_code: Option<Code>,
    #[serde(default)]
    pub packaging_type_code: Option<PackagingTypeCode>,
    #[serde(default)]
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

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_item() -> Item {
        Item {
            description: None,
            pack_quantity: None,
            pack_size_numeric: None,
            catalogue_indicator: None,
            name: None,
            hazardous_risk_indicator: None,
            additional_information: None,
            keyword: vec![],
            brand_name: vec![],
            model_name: vec![],
            buyers_item_identification: None,
            sellers_item_identification: None,
            manufacturers_item_identification: None,
            standard_item_identification: None,
            catalogue_item_identification: None,
            additional_item_identification: vec![],
            commodity_classification: vec![],
            item_instance: vec![],
            item_property: vec![],
            classified_tax_category: vec![],
            item_type_code: None,
            warranty_information: None,
            lifecycle_stage_code: None,
            lifecycle_stage_description: None,
        }
    }

    #[test]
    fn test_invoice_line_roundtrip() {
        use rust_decimal::Decimal;
        let mut item = empty_item();
        item.description = Some(Description::new("Widget"));
        item.name = Some(Name::new("Widget"));
        let line = InvoiceLine {
            id: ID::new("1"),
            uuid: None,
            note: vec![],
            invoiced_quantity: Some(InvoicedQuantity::new(Decimal::new(5, 0))),
            line_extension_amount: LineExtensionAmount::new(Decimal::new(10000, 2), "ZAR"),
            tax_inclusive_line_extension_amount: None,
            tax_point_date: None,
            accounting_cost_code: None,
            accounting_cost: None,
            payment_purpose_code: None,
            free_of_charge_indicator: None,
            invoice_period: vec![],
            order_line_reference: vec![],
            despatch_line_reference: vec![],
            receipt_line_reference: vec![],
            billing_reference: vec![],
            document_reference: vec![],
            pricing_reference: None,
            originator_party: None,
            delivery: vec![],
            payment_terms: vec![],
            allowance_charge: vec![],
            tax_total: vec![],
            withholding_tax_total: vec![],
            item,
            price: None,
            delivery_terms: None,
            sub_invoice_line: vec![],
            price_adjustment: None,
        };
        let json = serde_json::to_string(&line).unwrap();
        assert!(json.contains("Widget"));
        let line2: InvoiceLine = serde_json::from_str(&json).unwrap();
        assert_eq!(line.id.value(), line2.id.value());
    }
}
