#[derive(Debug, Deserialize, Serialize)]
pub struct TenderStatus {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::Identifier>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::Identifier>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::Identifier>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::Identifier>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(default, rename = "ContractFolderID")]
    pub contract_folder_id: Option<cct::Identifier>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
    #[serde(default, rename = "ContractName")]
    pub contract_name: Vec<cct::Text>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "ProcedureCode")]
    pub procedure_code: Option<cct::Code>,
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
    #[serde(default, rename = "TenderStatusInquiryDocumentReference")]
    pub tender_status_inquiry_document_reference: Vec<cac::DocumentReference>,
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
