// LineItem — UBL CAC aggregate
// A line in a document (order, invoice, etc.)
use crate::cbc::*;

/// A line in a business document.
/// UBL element: cac:LineItem
#[derive(Debug, Clone, Partialserde::Serialize, serde::Deserialize)]
pub struct LineItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_order_id: Option<SalesOrderID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<Uuid>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub note: Vec<Note>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_status_code: Option<LineStatusCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_extension_amount: Option<Amount>,
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
}
