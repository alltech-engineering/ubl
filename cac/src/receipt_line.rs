#[derive(Debug, Deserialize, Serialize)]
pub struct ReceiptLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "ReceivedQuantity")]
    pub received_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "ShortQuantity")]
    pub short_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "ShortageActionCode")]
    pub shortage_action_code: Option<cct::Code>,
    #[serde(default, rename = "RejectedQuantity")]
    pub rejected_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "RejectReasonCode")]
    pub reject_reason_code: Option<cct::Code>,
    #[serde(default, rename = "RejectReason")]
    pub reject_reason: Vec<cct::Text>,
    #[serde(default, rename = "RejectActionCode")]
    pub reject_action_code: Option<cct::Code>,
    #[serde(default, rename = "QuantityDiscrepancyCode")]
    pub quantity_discrepancy_code: Option<cct::Code>,
    #[serde(default, rename = "OversupplyQuantity")]
    pub oversupply_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "ReceivedDate")]
    pub received_date: Option<udt::DateTime>,
    #[serde(default, rename = "ReceivedTime")]
    pub received_time: Option<udt::DateTime>,
    #[serde(default, rename = "TimingComplaintCode")]
    pub timing_complaint_code: Option<cct::Code>,
    #[serde(default, rename = "TimingComplaint")]
    pub timing_complaint: Option<cct::Text>,
    #[serde(default, rename = "OrderLineReference")]
    pub order_line_reference: Option<OrderLineReference>,
    #[serde(default, rename = "DespatchLineReference")]
    pub despatch_line_reference: Vec<LineReference>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "Item")]
    pub item: Vec<Item>,
    #[serde(default, rename = "Shipment")]
    pub shipment: Vec<Shipment>,
}
