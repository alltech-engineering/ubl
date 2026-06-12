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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator_party: Option<Party>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_party: Option<Party>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub ordered_shipment: Vec<OrderedShipment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_reference: Option<PricingReference>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub allowance_charge: Vec<AllowanceCharge>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Price>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Item>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub sub_line_item: Vec<LineItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warranty_validity_period: Option<Period>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warranty_party: Option<Party>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<TaxTotal>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub item_price_extension: Vec<ItemPriceExtension>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub line_reference: Vec<LineReference>,
}

use super::allowance_charge::AllowanceCharge;
use super::delivery::Delivery;
use super::delivery_terms::DeliveryTerms;
use super::item::Item;
use super::item_price_extension::ItemPriceExtension;
use super::line::LineReference;
use super::line::OrderedShipment;
use super::party::Party;
use super::period::Period;
use super::price::Price;
use super::tax::PricingReference;
use super::tax::TaxTotal;
