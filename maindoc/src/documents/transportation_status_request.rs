#[derive(Debug, Deserialize, Serialize)]
/// A document requesting a Transportation Status report.
///
/// UBL Dictionary Entry Name: `Transportation Status Request. Details`
///
/// Generated from XSD type `TransportationStatusRequestType`.
pub struct TransportationStatusRequest {
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
/// A textual description of the document instance.
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
/// A code signifying the type of status requested in a Transportation Status document.
    #[serde(default, rename = "TransportationStatusTypeCode")]
    pub transportation_status_type_code: Option<cct::Code>,
/// The Party who sends this Document.
    #[serde(default, rename = "SenderParty")]
    pub sender_party: Option<cac::Party>,
/// The Party who receives this Document.
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: Option<cac::Party>,
/// A reference to the Transport Execution Plan associated with the transport service for which status
/// is requested.
    #[serde(default, rename = "TransportExecutionPlanDocumentReference")]
    pub transport_execution_plan_document_reference:
        Option<cac::DocumentReference>,
/// A consignment regarding which status is requested.
    #[serde(default, rename = "Consignment")]
    pub consignment: Vec<cac::Consignment>,
/// A reference to another document associated with this document.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<cac::DocumentReference>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// A location for which status is requested.
    #[serde(default, rename = "RequestedStatusLocation")]
    pub requested_status_location: Vec<cac::Location>,
/// A period for which status is requested.
    #[serde(default, rename = "RequestedStatusPeriod")]
    pub requested_status_period: Vec<cac::Period>,
}
