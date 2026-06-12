use serde::{Deserialize, Serialize};

// Re-exports for convenience

/// UBL 2.5 ProcurementStatusRequest document type.
/// A request for procurement process status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcurementStatusRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<ubl_common::cbc::UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<ubl_common::cbc::CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ubl_common::cbc::ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ubl_common::cbc::ProfileExecutionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ubl_common::cbc::ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<ubl_common::cbc::UUID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_folder_id: Option<ubl_common::cbc::ContractFolderID>,
    pub issue_date: ubl_common::cbc::IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<ubl_common::cbc::IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contract_name: Vec<ubl_common::cbc::ContractName>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<ubl_common::cbc::Note>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<ubl_common::cac::Signature>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contracting_party: Vec<ContractingParty>,
    pub economic_operator_party: EconomicOperatorParty,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tendering_process: Option<TenderingProcess>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub procurement_project: Option<ProcurementProject>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procurement_project_lot: Vec<ProcurementProjectLot>,
}

// ── Inline CAC types ──

/// UBL ContractingParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractingParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL EconomicOperatorParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EconomicOperatorParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL 2.5 TenderingProcess — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TenderingProcess {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ubl_common::cbc::ID>,
}

/// UBL 2.5 ProcurementProject — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcurementProject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ubl_common::cbc::ID>,
}

/// UBL 2.5 ProcurementProjectLot — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcurementProjectLot {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ubl_common::cbc::ID>,
}
