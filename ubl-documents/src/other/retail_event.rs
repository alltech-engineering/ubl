use serde::{Deserialize, Serialize};

// Re-exports for convenience

/// UBL 2.5 RetailEvent document type.
/// Retail event information (promotions, sales, etc.).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetailEvent {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<ubl_common::cbc::IssueDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<ubl_common::cbc::IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<ubl_common::cbc::Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retail_event_name: Option<ubl_common::cbc::RetailEventName>,
    pub retail_event_status_code: ubl_common::cbc::RetailEventStatusCode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seller_event_id: Option<ubl_common::cbc::SellerEventID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_event_id: Option<ubl_common::cbc::BuyerEventID>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<ubl_common::cbc::Description>,
    pub period: ubl_common::cac::Period,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub original_document_reference: Vec<OriginalDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<ubl_common::cac::Signature>,
    pub sender_party: SenderParty,
    pub receiver_party: ReceiverParty,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_customer_party: Option<BuyerCustomerParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seller_supplier_party: Option<SellerSupplierParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event_comment: Vec<EventComment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub promotional_event: Option<PromotionalEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub miscellaneous_event: Option<MiscellaneousEvent>,
}

// ── Inline CAC types ──

/// UBL 2.5 OriginalDocumentReference — TODO: define fields from CAC schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OriginalDocumentReference {
    // TODO: Define fields from UBL 2.5 CAC schema
}

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

/// UBL BuyerCustomerParty — a CustomerParty playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuyerCustomerParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::CustomerParty>,
    // TODO: Add role-specific fields from UBL 2.5 CAC schema
}

/// UBL SellerSupplierParty — a SupplierParty playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SellerSupplierParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::SupplierParty>,
    // TODO: Add role-specific fields from UBL 2.5 CAC schema
}

/// UBL 2.5 EventComment — TODO: define fields from CAC schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventComment {
    // TODO: Define fields from UBL 2.5 CAC schema
}

/// UBL 2.5 PromotionalEvent — TODO: define fields from CAC schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromotionalEvent {
    // TODO: Define fields from UBL 2.5 CAC schema
}

/// UBL 2.5 MiscellaneousEvent — TODO: define fields from CAC schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MiscellaneousEvent {
    // TODO: Define fields from UBL 2.5 CAC schema
}
