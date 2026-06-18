#[derive(Debug, Deserialize, Serialize)]
pub struct DespatchLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "LineStatusCode")]
    pub line_status_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "DeliveredQuantity")]
    pub delivered_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "BackorderQuantity")]
    pub backorder_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "BackorderReason")]
    pub backorder_reason: Vec<super::cct::TextType>,
    #[serde(default, rename = "OutstandingQuantity")]
    pub outstanding_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "OutstandingReason")]
    pub outstanding_reason: Vec<super::cct::TextType>,
    #[serde(default, rename = "OversupplyQuantity")]
    pub oversupply_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<super::cct::TextType>,
    #[serde(default, rename = "OrderLineReference")]
    pub order_line_reference: Vec<OrderLineReference>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
    #[serde(rename = "Item")]
    pub item: Item,
    #[serde(default, rename = "Shipment")]
    pub shipment: Vec<Shipment>,
    #[serde(default, rename = "SubDespatchLine")]
    pub sub_despatch_line: Vec<DespatchLine>,
}
