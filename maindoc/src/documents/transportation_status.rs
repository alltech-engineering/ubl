#[derive(Debug, Deserialize, Serialize)]
pub struct TransportationStatus {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CarrierAssignedID")]
    pub carrier_assigned_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTimeType>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::TextType>,
    #[serde(default, rename = "ShippingOrderID")]
    pub shipping_order_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "OtherInstruction")]
    pub other_instruction: Option<cct::TextType>,
    #[serde(default, rename = "TransportationStatusTypeCode")]
    pub transportation_status_type_code: Option<cct::CodeType>,
    #[serde(default, rename = "TransportExecutionStatusCode")]
    pub transport_execution_status_code: Option<cct::CodeType>,
    #[serde(default, rename = "Consignment")]
    pub consignment: Vec<cac::Consignment>,
    #[serde(default, rename = "TransportEvent")]
    pub transport_event: Vec<cac::TransportEvent>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(default, rename = "SenderParty")]
    pub sender_party: Option<cac::Party>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: Option<cac::Party>,
    #[serde(default, rename = "TransportationStatusRequestDocumentReference")]
    pub transportation_status_request_document_reference:
        Option<cac::DocumentReference>,
    #[serde(default, rename = "TransportExecutionPlanDocumentReference")]
    pub transport_execution_plan_document_reference:
        Option<cac::DocumentReference>,
    #[serde(default, rename = "UpdatedPickupTransportEvent")]
    pub updated_pickup_transport_event: Option<cac::TransportEvent>,
    #[serde(default, rename = "UpdatedDeliveryTransportEvent")]
    pub updated_delivery_transport_event: Option<cac::TransportEvent>,
    #[serde(default, rename = "StatusLocation")]
    pub status_location: Vec<cac::Location>,
    #[serde(default, rename = "StatusPeriod")]
    pub status_period: Vec<cac::Period>,
}
