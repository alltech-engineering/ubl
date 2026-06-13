use serde::{Deserialize, Serialize};

// Re-exports for convenience

/// UBL 2.5 ProcurementStatus document type.
/// Status of a procurement process.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcurementStatus {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub procedure_code: Option<ubl_common::cbc::ProcedureCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tender_submission_deadline_period: Option<TenderSubmissionDeadlinePeriod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invitation_submission_period: Option<InvitationSubmissionPeriod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participation_request_reception_period: Option<ParticipationRequestReceptionPeriod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub procurement_legislation_document_reference: Option<ProcurementLegislationDocumentReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fiscal_legislation_document_reference: Option<FiscalLegislationDocumentReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environmental_legislation_document_reference:
        Option<EnvironmentalLegislationDocumentReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_legislation_document_reference: Option<EmploymentLegislationDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procedure_status_request_document_reference: Vec<ProcedureStatusRequestDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<ubl_common::cac::Signature>,
    pub contracting_party: ContractingParty,
    pub economic_operator_party: EconomicOperatorParty,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_provider_party: Option<DocumentProviderParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tender_recipient_party: Option<TenderRecipientParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub procurement_project: Option<ProcurementProject>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procurement_project_lot: Vec<ProcurementProjectLot>,
}

// ── Inline CAC types ──

/// UBL TenderSubmissionDeadlinePeriod — a Period with this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TenderSubmissionDeadlinePeriod {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<ubl_common::cac::Period>,
}

/// UBL InvitationSubmissionPeriod — a Period with this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvitationSubmissionPeriod {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<ubl_common::cac::Period>,
}

/// UBL ParticipationRequestReceptionPeriod — a Period with this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParticipationRequestReceptionPeriod {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<ubl_common::cac::Period>,
}

/// UBL 2.5 ProcurementLegislationDocumentReference — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcurementLegislationDocumentReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<ubl_common::cac::document_reference::DocumentReference>,
}

/// UBL 2.5 FiscalLegislationDocumentReference — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FiscalLegislationDocumentReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<ubl_common::cac::document_reference::DocumentReference>,
}

/// UBL 2.5 EnvironmentalLegislationDocumentReference — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentalLegislationDocumentReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<ubl_common::cac::document_reference::DocumentReference>,
}

/// UBL 2.5 EmploymentLegislationDocumentReference — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmploymentLegislationDocumentReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<ubl_common::cac::document_reference::DocumentReference>,
}

/// UBL 2.5 ProcedureStatusRequestDocumentReference — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcedureStatusRequestDocumentReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<ubl_common::cac::document_reference::DocumentReference>,
}

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

/// UBL DocumentProviderParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentProviderParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL TenderRecipientParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TenderRecipientParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
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
