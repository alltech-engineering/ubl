#[derive(Debug, Deserialize, Serialize)]
/// A document to circulate reports of transportation status or changes in status (events) among a group
/// of participants.
///
/// UBL Dictionary Entry Name: `Transportation Status. Details`
///
/// Generated from XSD type `TransportationStatusType`.
pub struct TransportationStatus {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
/// Identifies the earliest version of the UBL 2 schema for this document type that defines all of the
/// elements that might be encountered in the current instance.
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::Identifier>,
/// Identifies a user-defined customization of UBL for a specific use.
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::Identifier>,
/// Identifies a user-defined profile of the customization of UBL being used.
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::Identifier>,
/// Identifies an instance of executing a profile, to associate all transactions in a collaboration.
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::Identifier>,
/// An identifier for this document, assigned by the sender.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// A reference number assigned by a carrier or its agent to identify a specific shipment, such as a
/// booking reference number when cargo space is reserved prior to loading.
    #[serde(default, rename = "CarrierAssignedID")]
    pub carrier_assigned_id: Option<cct::Identifier>,
/// A universally unique identifier for an instance of this document.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// The date, assigned by the sender, on which this document was issued.
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
/// The time, assigned by the sender, at which this document was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// Text, assigned by the sender, that identifies this document to business users.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// A textual description of transportation status.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// A reference number for a shipping order.
    #[serde(default, rename = "ShippingOrderID")]
    pub shipping_order_id: Option<cct::Identifier>,
/// An instruction regarding this message.
    #[serde(default, rename = "OtherInstruction")]
    pub other_instruction: Option<cct::Text>,
/// A code signifying the type of status provided in a Transportation Status document.
    #[serde(default, rename = "TransportationStatusTypeCode")]
    pub transportation_status_type_code: Option<cct::Code>,
/// A code signifying the overall status of transport service execution.
    #[serde(default, rename = "TransportExecutionStatusCode")]
    pub transport_execution_status_code: Option<cct::Code>,
/// A consignment associated with this Transportation Status report.
    #[serde(default, rename = "Consignment")]
    pub consignment: Vec<cac::Consignment>,
/// Any additional events associated with this Transportation Status report that are not defined
/// elsewhere in this document.
    #[serde(default, rename = "TransportEvent")]
    pub transport_event: Vec<cac::TransportEvent>,
/// A reference to another document associated with this document.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<cac::DocumentReference>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The Party who sends this Transportation Status Report.
    #[serde(default, rename = "SenderParty")]
    pub sender_party: Option<cac::Party>,
/// The Party who receives this Transportation Status Report.
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: Option<cac::Party>,
/// A reference to the Transportation Status Request to which this report is a response.
    #[serde(default, rename = "TransportationStatusRequestDocumentReference")]
    pub transportation_status_request_document_reference:
        Option<cac::DocumentReference>,
/// A reference to the Transport Execution Plan associated with the transport service whose status is
/// being reported.
    #[serde(default, rename = "TransportExecutionPlanDocumentReference")]
    pub transport_execution_plan_document_reference:
        Option<cac::DocumentReference>,
/// Update of the original plan regarding a pickup of goods.
    #[serde(default, rename = "UpdatedPickupTransportEvent")]
    pub updated_pickup_transport_event: Option<cac::TransportEvent>,
/// Update of the original plan regarding a delivery.
    #[serde(default, rename = "UpdatedDeliveryTransportEvent")]
    pub updated_delivery_transport_event: Option<cac::TransportEvent>,
/// Locations associated with this Transportation Status report.
    #[serde(default, rename = "StatusLocation")]
    pub status_location: Vec<cac::Location>,
/// A period for which status is provided.
    #[serde(default, rename = "StatusPeriod")]
    pub status_period: Vec<cac::Period>,
}
