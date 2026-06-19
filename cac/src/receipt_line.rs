#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line in a Receipt Advice.
///
/// UBL Dictionary Entry Name: `Receipt Line. Details`
///
/// Generated from XSD type `ReceiptLineType`.
pub struct ReceiptLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this receipt line.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// A universally unique identifier for this receipt line.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// The quantity received.
    #[serde(default, rename = "ReceivedQuantity")]
    pub received_quantity: Option<cct::Quantity>,
/// The quantity received short; the difference between the quantity reported despatched and the
/// quantity actually received.
    #[serde(default, rename = "ShortQuantity")]
    pub short_quantity: Option<cct::Quantity>,
/// A code signifying the action that the delivery party wishes the despatch party to take as the result
/// of a shortage.
    #[serde(default, rename = "ShortageActionCode")]
    pub shortage_action_code: Option<cct::Code>,
/// The quantity rejected.
    #[serde(default, rename = "RejectedQuantity")]
    pub rejected_quantity: Option<cct::Quantity>,
/// The reason for a rejection, expressed as a code.
    #[serde(default, rename = "RejectReasonCode")]
    pub reject_reason_code: Option<cct::Code>,
/// The reason for a rejection, expressed as text.
    #[serde(default, rename = "RejectReason")]
    pub reject_reason: Vec<cct::Text>,
/// A code signifying the action that the delivery party wishes the despatch party to take as the result
/// of a rejection.
    #[serde(default, rename = "RejectActionCode")]
    pub reject_action_code: Option<cct::Code>,
/// A code signifying the type of a discrepancy in quantity.
    #[serde(default, rename = "QuantityDiscrepancyCode")]
    pub quantity_discrepancy_code: Option<cct::Code>,
/// The quantity over-supplied, i.e., the quantity over and above the quantity ordered.
    #[serde(default, rename = "OversupplyQuantity")]
    pub oversupply_quantity: Option<cct::Quantity>,
/// The date on which the goods or services were received.
    #[serde(default, rename = "ReceivedDate")]
    pub received_date: Option<udt::DateTime>,
/// The time at which the goods or services were received.
    #[serde(default, rename = "ReceivedTime")]
    pub received_time: Option<udt::DateTime>,
/// A complaint about the timing of delivery, expressed as a code.
    #[serde(default, rename = "TimingComplaintCode")]
    pub timing_complaint_code: Option<cct::Code>,
/// A complaint about the timing of delivery, expressed as text.
    #[serde(default, rename = "TimingComplaint")]
    pub timing_complaint: Option<cct::Text>,
/// A reference to the order line associated with this receipt line.
    #[serde(default, rename = "OrderLineReference")]
    pub order_line_reference: Option<OrderLineReference>,
/// A reference to a despatch line associated with this receipt line.
    #[serde(default, rename = "DespatchLineReference")]
    pub despatch_line_reference: Vec<LineReference>,
/// A reference to a document associated with this receipt line.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
/// An item associated with this receipt line.
    #[serde(default, rename = "Item")]
    pub item: Vec<Item>,
/// A shipment associated with this receipt line.
    #[serde(default, rename = "Shipment")]
    pub shipment: Vec<Shipment>,
}
