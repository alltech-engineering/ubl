// LineItem — UBL CAC aggregate
// A line in a document (order, invoice, etc.)
use crate::cbc::*;

/// A line in a business document.
/// UBL element: cac:LineItem
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LineItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_order_id: Option<SalesOrderID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub note: Vec<Note>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_status_code: Option<LineStatusCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_extension_amount: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_inclusive_line_extension_amount: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax_amount: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_quantity: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_quantity: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_backorder_quantity: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_backorder_quantity: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inspection_method_code: Option<InspectionMethodCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_delivery_indicator: Option<PartialDeliveryIndicator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_order_allowed_indicator: Option<BackOrderAllowedIndicator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost_code: Option<AccountingCostCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost: Option<AccountingCost>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub warranty_information: Vec<WarrantyInformation>,
    // CAC references
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub delivery: Vec<Delivery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_terms: Option<DeliveryTerms>,
    // TODO: cac:OriginatorParty — Party, not yet wired
    // TODO: cac:BeneficiaryParty — Party, not yet wired
    // TODO: cac:OrderedShipment — Shipment, not yet wired
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_reference: Option<PricingReference>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub allowance_charge: Vec<AllowanceCharge>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Price>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Item>,
    // TODO: cac:SubLineItem — recursive LineItem, not yet wired
    // TODO: cac:WarrantyValidityPeriod — Period, not yet wired
    // TODO: cac:WarrantyParty — Party, not yet wired
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<TaxTotal>,
    // TODO: cac:ItemPriceExtension — not yet implemented
    // TODO: cac:LineReference — not yet implemented
}

use super::delivery::Delivery;
use super::delivery_terms::DeliveryTerms;
use super::tax::PricingReference;
use super::allowance_charge::AllowanceCharge;
use super::price::Price;
use super::item::Item;
use super::tax::TaxTotal;
