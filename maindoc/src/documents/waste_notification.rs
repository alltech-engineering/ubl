#[derive(Debug, Deserialize, Serialize)]
pub struct WasteNotification {
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
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTimeType>,
    #[serde(default, rename = "WasteNotificationTypeCode")]
    pub waste_notification_type_code: Option<cct::CodeType>,
    #[serde(default, rename = "ConsignmentQuantity")]
    pub consignment_quantity: Option<cct::QuantityType>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::Party,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::Party,
    #[serde(rename = "NotifierParty")]
    pub notifier_party: cac::Party,
    #[serde(default, rename = "CustomsParty")]
    pub customs_party: Vec<cac::Party>,
    #[serde(default, rename = "DisposalFacilityParty")]
    pub disposal_facility_party: Option<cac::Party>,
    #[serde(default, rename = "RecoveryFacilityParty")]
    pub recovery_facility_party: Option<cac::Party>,
    #[serde(rename = "WasteProducerParty")]
    pub waste_producer_party: cac::Party,
    #[serde(rename = "Shipment")]
    pub shipment: cac::Shipment,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<cac::DocumentReference>,
}
