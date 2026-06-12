use serde::{Deserialize, Serialize};

// Re-exports for convenience

/// UBL 2.5 WasteNotification document type.
/// Waste shipment notification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WasteNotification {
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
    pub waste_notification_type_code: Option<ubl_common::cbc::WasteNotificationTypeCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consignment_quantity: Option<ubl_common::cbc::ConsignmentQuantity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<ubl_common::cac::Signature>,
    pub sender_party: SenderParty,
    pub receiver_party: ReceiverParty,
    pub notifier_party: NotifierParty,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub customs_party: Vec<CustomsParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disposal_facility_party: Option<DisposalFacilityParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_facility_party: Option<RecoveryFacilityParty>,
    pub waste_producer_party: WasteProducerParty,
    pub shipment: ubl_common::cac::Shipment,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document_reference: Vec<ubl_common::cac::DocumentReference>,
}

// ── Inline CAC types ──

/// UBL SenderParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SenderParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL ReceiverParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReceiverParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL NotifierParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotifierParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL CustomsParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomsParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL DisposalFacilityParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisposalFacilityParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL RecoveryFacilityParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecoveryFacilityParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL WasteProducerParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WasteProducerParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}
