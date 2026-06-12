use serde::{Deserialize, Serialize};

// Re-exports for convenience

/// UBL 2.5 WasteMovement document type.
/// Waste consignment movement document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WasteMovement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<ubl_common::cbc::UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<ubl_common::cbc::CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ubl_common::cbc::ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ubl_common::cbc::ProfileExecutionID>,
    pub id: ubl_common::cbc::ID,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<ubl_common::cbc::UUID>,
    pub issue_date: ubl_common::cbc::IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<ubl_common::cbc::IssueTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waste_movement_type_code: Option<ubl_common::cbc::WasteMovementTypeCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequence_number_id: Option<ubl_common::cbc::SequenceNumberID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consignment_quantity: Option<ubl_common::cbc::ConsignmentQuantity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<ubl_common::cac::Signature>,
    pub sender_party: SenderParty,
    pub receiver_party: ReceiverParty,
    pub notifier_party: NotifierParty,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disposal_facility_party: Option<DisposalFacilityParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_facility_party: Option<RecoveryFacilityParty>,
    pub waste_producer_party: WasteProducerParty,
    pub shipment: ubl_common::cac::Shipment,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waste_notification_document_reference: Option<WasteNotificationDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub weight_statement_document_reference: Vec<WeightStatementDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document_reference: Vec<ubl_common::cac::DocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document_distribution: Vec<ubl_common::cac::DocumentDistribution>,
}

// ── Inline CAC types ──

/// UBL SenderParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SenderParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
    // TODO: Add role-specific fields from UBL 2.5 CAC schema
}

/// UBL ReceiverParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReceiverParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
    // TODO: Add role-specific fields from UBL 2.5 CAC schema
}

/// UBL NotifierParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotifierParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
    // TODO: Add role-specific fields from UBL 2.5 CAC schema
}

/// UBL DisposalFacilityParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisposalFacilityParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
    // TODO: Add role-specific fields from UBL 2.5 CAC schema
}

/// UBL RecoveryFacilityParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecoveryFacilityParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
    // TODO: Add role-specific fields from UBL 2.5 CAC schema
}

/// UBL WasteProducerParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WasteProducerParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
    // TODO: Add role-specific fields from UBL 2.5 CAC schema
}

/// UBL 2.5 WasteNotificationDocumentReference — TODO: define fields from CAC schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WasteNotificationDocumentReference {
    // TODO: Define fields from UBL 2.5 CAC schema
}

/// UBL 2.5 WeightStatementDocumentReference — TODO: define fields from CAC schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeightStatementDocumentReference {
    // TODO: Define fields from UBL 2.5 CAC schema
}
