use serde::{Deserialize, Serialize};

// Re-exports for convenience

/// UBL 2.5 ImportCustomsDeclaration document type.
/// Import customs declaration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportCustomsDeclaration {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<ubl_common::cbc::IssueDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<ubl_common::cbc::IssueTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_code: Option<ubl_common::cbc::TypeCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_type_code: Option<ubl_common::cbc::SubTypeCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nature_of_transaction_code: Option<ubl_common::cbc::NatureOfTransactionCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<ubl_common::cbc::Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<ubl_common::cbc::VersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<ValidityPeriod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customs_exit_office_location: Option<CustomsExitOfficeLocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jurisdiction_region_address: Option<JurisdictionRegionAddress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub importer_party: Option<ImporterParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consignor_party: Option<ConsignorParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consignee_party: Option<ConsigneeParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freight_forwarder_party: Option<FreightForwarderParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customs_party: Option<CustomsParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notifier_party: Option<NotifierParty>,
    pub shipment: ubl_common::cac::Shipment,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_customs_declaration: Option<PreviousCustomsDeclaration>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_document_reference: Vec<AdditionalDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<ubl_common::cac::Signature>,
}

// ── Inline CAC types ──

/// UBL ValidityPeriod — a Period with this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidityPeriod {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<ubl_common::cac::Period>,
}

/// UBL 2.5 CustomsExitOfficeLocation — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomsExitOfficeLocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<ubl_common::cac::location::Location>,
}

/// UBL 2.5 JurisdictionRegionAddress — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JurisdictionRegionAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<ubl_common::cac::address::Address>,
}

/// UBL ImporterParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImporterParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL ConsignorParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsignorParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL ConsigneeParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsigneeParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL FreightForwarderParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FreightForwarderParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL CustomsParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomsParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL NotifierParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotifierParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL 2.5 PreviousCustomsDeclaration — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreviousCustomsDeclaration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ubl_common::cbc::ID>,
}

/// UBL 2.5 AdditionalDocumentReference — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionalDocumentReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<ubl_common::cac::document_reference::DocumentReference>,
}
