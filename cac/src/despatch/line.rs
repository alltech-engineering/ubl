#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line in a Despatch Advice.
///
/// UBL Dictionary Entry Name: `Despatch Line. Details`
///
/// Generated from XSD type `DespatchLineType`.
pub struct DespatchLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this despatch line.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// A universally unique identifier for this despatch line.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// A code signifying the status of this despatch line with respect to its original state.
    #[serde(default, rename = "LineStatusCode")]
    pub line_status_code: Option<cct::Code>,
/// The quantity despatched (picked up).
    #[serde(default, rename = "DeliveredQuantity")]
    pub delivered_quantity: Option<cct::Quantity>,
/// The quantity on back order at the supplier.
    #[serde(default, rename = "BackorderQuantity")]
    pub backorder_quantity: Option<cct::Quantity>,
/// The reason for the back order.
    #[serde(default, rename = "BackorderReason")]
    pub backorder_reason: Vec<cct::Text>,
/// The quantity outstanding (which will follow in a later despatch).
    #[serde(default, rename = "OutstandingQuantity")]
    pub outstanding_quantity: Option<cct::Quantity>,
/// The reason for the outstanding quantity.
    #[serde(default, rename = "OutstandingReason")]
    pub outstanding_reason: Vec<cct::Text>,
/// The quantity over-supplied, i.e., the quantity over and above that ordered.
    #[serde(default, rename = "OversupplyQuantity")]
    pub oversupply_quantity: Option<cct::Quantity>,
/// The accounting cost centre, applied to the Despatch Advice Line, expressed as a code.
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<cct::Code>,
/// The accounting cost centre, applied to the Despatch Advice Line, expressed as text.
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<cct::Text>,
/// A reference to an order line associated with this despatch line.
    #[serde(default, rename = "OrderLineReference")]
    pub order_line_reference: Vec<crate::OrderLineReference>,
/// A reference to a document associated with this despatch line.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<crate::DocumentReference>,
/// The item associated with this despatch line.
    #[serde(rename = "Item")]
    pub item: crate::Item,
/// A shipment associated with this despatch line.
    #[serde(default, rename = "Shipment")]
    pub shipment: Vec<crate::Shipment>,
/// A despatch line subsidiary to this despatch line.
    #[serde(default, rename = "SubDespatchLine")]
    pub sub_despatch_line: Vec<DespatchLine>,
}
