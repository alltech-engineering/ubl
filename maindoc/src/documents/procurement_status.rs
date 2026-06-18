#[derive(Debug, Deserialize, Serialize)]
pub struct ProcurementStatus {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::IdentifierType>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::IdentifierType>,
    #[serde(default, rename = "ContractFolderID")]
    pub contract_folder_id: Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTimeType>,
    #[serde(default, rename = "ContractName")]
    pub contract_name: Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::TextType>,
    #[serde(default, rename = "ProcedureCode")]
    pub procedure_code: Option<cct::CodeType>,
    #[serde(default, rename = "TenderSubmissionDeadlinePeriod")]
    pub tender_submission_deadline_period: Option<cac::Period>,
    #[serde(default, rename = "InvitationSubmissionPeriod")]
    pub invitation_submission_period: Option<cac::Period>,
    #[serde(default, rename = "ParticipationRequestReceptionPeriod")]
    pub participation_request_reception_period: Option<cac::Period>,
    #[serde(default, rename = "ProcurementLegislationDocumentReference")]
    pub procurement_legislation_document_reference:
        Option<cac::DocumentReference>,
    #[serde(default, rename = "FiscalLegislationDocumentReference")]
    pub fiscal_legislation_document_reference: Option<cac::DocumentReference>,
    #[serde(default, rename = "EnvironmentalLegislationDocumentReference")]
    pub environmental_legislation_document_reference:
        Option<cac::DocumentReference>,
    #[serde(default, rename = "EmploymentLegislationDocumentReference")]
    pub employment_legislation_document_reference:
        Option<cac::DocumentReference>,
    #[serde(default, rename = "ProcedureStatusRequestDocumentReference")]
    pub procedure_status_request_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(rename = "ContractingParty")]
    pub contracting_party: cac::ContractingParty,
    #[serde(rename = "EconomicOperatorParty")]
    pub economic_operator_party: cac::EconomicOperatorParty,
    #[serde(default, rename = "DocumentProviderParty")]
    pub document_provider_party: Option<cac::Party>,
    #[serde(default, rename = "TenderRecipientParty")]
    pub tender_recipient_party: Option<cac::Party>,
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: Option<cac::ProcurementProject>,
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: Vec<cac::ProcurementProjectLot>,
}
