#[derive(Debug, Deserialize, Serialize)]
pub struct ReceiptLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "ReceivedQuantity")]
    pub received_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "ShortQuantity")]
    pub short_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "ShortageActionCode")]
    pub shortage_action_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "RejectedQuantity")]
    pub rejected_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "RejectReasonCode")]
    pub reject_reason_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "RejectReason")]
    pub reject_reason: Vec<super::cct::TextType>,
    #[serde(default, rename = "RejectActionCode")]
    pub reject_action_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "QuantityDiscrepancyCode")]
    pub quantity_discrepancy_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "OversupplyQuantity")]
    pub oversupply_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "ReceivedDate")]
    pub received_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ReceivedTime")]
    pub received_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "TimingComplaintCode")]
    pub timing_complaint_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "TimingComplaint")]
    pub timing_complaint: Option<super::cct::TextType>,
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
