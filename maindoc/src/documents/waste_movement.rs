#[derive(Debug, Deserialize, Serialize)]
pub struct WasteMovement {
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
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
    #[serde(default, rename = "WasteMovementTypeCode")]
    pub waste_movement_type_code: Option<cct::Code>,
    #[serde(default, rename = "SequenceNumberID")]
    pub sequence_number_id: Option<cct::Identifier>,
    #[serde(default, rename = "ConsignmentQuantity")]
    pub consignment_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::Party,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::Party,
    #[serde(rename = "NotifierParty")]
    pub notifier_party: cac::Party,
    #[serde(default, rename = "DisposalFacilityParty")]
    pub disposal_facility_party: Option<cac::Party>,
    #[serde(default, rename = "RecoveryFacilityParty")]
    pub recovery_facility_party: Option<cac::Party>,
    #[serde(rename = "WasteProducerParty")]
    pub waste_producer_party: cac::Party,
    #[serde(rename = "Shipment")]
    pub shipment: cac::Shipment,
    #[serde(default, rename = "WasteNotificationDocumentReference")]
    pub waste_notification_document_reference: Option<cac::DocumentReference>,
    #[serde(default, rename = "WeightStatementDocumentReference")]
    pub weight_statement_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "DocumentDistribution")]
    pub document_distribution: Vec<cac::DocumentDistribution>,
}
