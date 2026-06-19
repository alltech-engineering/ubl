#[derive(Debug, Deserialize, Serialize)]
pub struct DespatchLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "LineStatusCode")]
    pub line_status_code: Option<cct::Code>,
    #[serde(default, rename = "DeliveredQuantity")]
    pub delivered_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "BackorderQuantity")]
    pub backorder_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "BackorderReason")]
    pub backorder_reason: Vec<cct::Text>,
    #[serde(default, rename = "OutstandingQuantity")]
    pub outstanding_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "OutstandingReason")]
    pub outstanding_reason: Vec<cct::Text>,
    #[serde(default, rename = "OversupplyQuantity")]
    pub oversupply_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<cct::Code>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<cct::Text>,
    #[serde(default, rename = "OrderLineReference")]
    pub order_line_reference: Vec<crate::OrderLineReference>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<crate::DocumentReference>,
    #[serde(rename = "Item")]
    pub item: crate::Item,
    #[serde(default, rename = "Shipment")]
    pub shipment: Vec<crate::Shipment>,
    #[serde(default, rename = "SubDespatchLine")]
    pub sub_despatch_line: Vec<DespatchLine>,
}
