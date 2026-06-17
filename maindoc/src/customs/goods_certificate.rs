use serde::{Deserialize, Serialize};

// Re-exports for convenience

/// UBL 2.5 GoodsCertificate document type.
/// Certificate for goods (origin, quality, inspection).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoodsCertificate {
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<ubl_common::cbc::Description>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<ubl_common::cbc::Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<ubl_common::cbc::VersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<ValidityPeriod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applicable_territory_address: Option<ApplicableTerritoryAddress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exporter_party: Option<ExporterParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub importer_party: Option<ImporterParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warehouse_party: Option<WarehouseParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consignor_party: Option<ConsignorParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consignee_party: Option<ConsigneeParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freight_forwarder_party: Option<FreightForwarderParty>,
    pub issuer_party: IssuerParty,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_authority_party: Option<LegalAuthorityParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applicant_party: Option<ApplicantParty>,
    pub shipment: ubl_common::cac::Shipment,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attestation: Vec<Attestation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub goods_processing: Vec<GoodsProcessing>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_document_reference: Option<OriginalDocumentReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_document_reference: Option<PreviousDocumentReference>,
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

/// UBL 2.5 ApplicableTerritoryAddress — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicableTerritoryAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<ubl_common::cac::address::Address>,
}

/// UBL ExporterParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExporterParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL ImporterParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImporterParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL WarehouseParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WarehouseParty {
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

/// UBL IssuerParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssuerParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL LegalAuthorityParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegalAuthorityParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL ApplicantParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicantParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL 2.5 Attestation — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attestation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ubl_common::cbc::ID>,
}

/// UBL 2.5 GoodsProcessing — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoodsProcessing {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ubl_common::cbc::ID>,
}

/// UBL 2.5 OriginalDocumentReference — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OriginalDocumentReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<ubl_common::cac::document_reference::DocumentReference>,
}

/// UBL 2.5 PreviousDocumentReference — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreviousDocumentReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<ubl_common::cac::document_reference::DocumentReference>,
}

/// UBL 2.5 AdditionalDocumentReference — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionalDocumentReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<ubl_common::cac::document_reference::DocumentReference>,
}
