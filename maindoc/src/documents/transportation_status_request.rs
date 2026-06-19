#[derive(Debug, Deserialize, Serialize)]
pub struct TransportationStatusRequest {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::Identifier>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::Identifier>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::Identifier>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::Identifier>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "CarrierAssignedID")]
    pub carrier_assigned_id: Option<cct::Identifier>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "ShippingOrderID")]
    pub shipping_order_id: Option<cct::Identifier>,
    #[serde(default, rename = "OtherInstruction")]
    pub other_instruction: Option<cct::Text>,
    #[serde(default, rename = "TransportationStatusTypeCode")]
    pub transportation_status_type_code: Option<cct::Code>,
    #[serde(default, rename = "SenderParty")]
    pub sender_party: Option<cac::Party>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: Option<cac::Party>,
    #[serde(default, rename = "TransportExecutionPlanDocumentReference")]
    pub transport_execution_plan_document_reference:
        Option<cac::DocumentReference>,
    #[serde(default, rename = "Consignment")]
    pub consignment: Vec<cac::Consignment>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(default, rename = "RequestedStatusLocation")]
    pub requested_status_location: Vec<cac::Location>,
    #[serde(default, rename = "RequestedStatusPeriod")]
    pub requested_status_period: Vec<cac::Period>,
}
