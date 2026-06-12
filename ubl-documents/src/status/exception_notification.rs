use serde::{Deserialize, Serialize};

// Re-exports for convenience

/// UBL 2.5 ExceptionNotification document type.
/// Notification of an exception condition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExceptionNotification {
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
    pub copy_indicator: Option<ubl_common::cbc::CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<ubl_common::cbc::UUID>,
    pub issue_date: ubl_common::cbc::IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<ubl_common::cbc::IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<ubl_common::cbc::Note>,
    pub exception_observation_period: ExceptionObservationPeriod,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document_reference: Vec<ubl_common::cac::DocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<ubl_common::cac::Signature>,
    pub sender_party: SenderParty,
    pub receiver_party: ReceiverParty,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_customer_party: Option<BuyerCustomerParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seller_supplier_party: Option<SellerSupplierParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exception_notification_line: Vec<ExceptionNotificationLine>,
}

// ── Inline CAC types ──

/// UBL ExceptionObservationPeriod — a Period with this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExceptionObservationPeriod {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<ubl_common::cac::Period>,
}

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

/// UBL BuyerCustomerParty — a CustomerParty playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuyerCustomerParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::CustomerParty>,
}

/// UBL SellerSupplierParty — a SupplierParty playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SellerSupplierParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::SupplierParty>,
}

/// UBL 2.5 ExceptionNotificationLine — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExceptionNotificationLine {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ubl_common::cbc::ID>,
}
