use serde::{Deserialize, Serialize};

// Re-exports for convenience

/// UBL 2.5 Forecast document type.
/// A demand or supply forecast.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Forecast {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<ubl_common::cbc::VersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub based_on_consensus_indicator: Option<ubl_common::cbc::BasedOnConsensusIndicator>,
    pub forecast_purpose_code: ubl_common::cbc::ForecastPurposeCode,
    pub forecast_period: ForecastPeriod,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_document_reference: Vec<AdditionalDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<ubl_common::cac::Signature>,
    pub sender_party: SenderParty,
    pub receiver_party: ReceiverParty,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_customer_party: Option<BuyerCustomerParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seller_supplier_party: Option<SellerSupplierParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub forecast_line: Vec<ForecastLine>,
}

// ── Inline CAC types ──

/// UBL ForecastPeriod — a Period with this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForecastPeriod {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<ubl_common::cac::Period>,
}

/// UBL 2.5 AdditionalDocumentReference — TODO: define fields from CAC schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionalDocumentReference {
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

/// UBL 2.5 ForecastLine — TODO: define fields from CAC schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForecastLine {
    // TODO: Define fields from UBL 2.5 CAC schema
}
