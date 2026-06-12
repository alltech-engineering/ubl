use serde::{Deserialize, Serialize};

// Re-exports for convenience

/// UBL 2.5 ExportCustomsDeclaration document type.
/// Export customs declaration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportCustomsDeclaration {
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
    pub export_type_code: Option<ubl_common::cbc::ExportTypeCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub export_reason_code: Option<ubl_common::cbc::ExportReasonCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<ubl_common::cbc::Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<ubl_common::cbc::VersionID>,
    pub exporter_party: ExporterParty,
    pub customs_declaration: ubl_common::cac::CustomsDeclaration,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<ubl_common::cac::Signature>,
}

// ── Inline CAC types ──

/// UBL ExporterParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExporterParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}
