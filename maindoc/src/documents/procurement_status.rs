#[derive(Debug, Deserialize, Serialize)]
/// A document sent by a Contracting Party to an Economic Operator describing the status of a
/// procurement procedure, Project, or Lot.
///
/// UBL Dictionary Entry Name: `Procurement Status. Details`
///
/// Generated from XSD type `ProcurementStatusType`.
pub struct ProcurementStatus {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
/// Identifies the earliest version of the UBL 2 schema for this document type that defines all of the
/// elements that might be encountered in the current instance.
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::Identifier>,
/// Identifies a user-defined customization of UBL for a specific use.
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::Identifier>,
/// Identifies a user-defined profile of the customization of UBL being used.
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::Identifier>,
/// Identifies an instance of executing a profile, to associate all transactions in a collaboration.
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::Identifier>,
/// An identifier for this document, assigned by the sender.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A universally unique identifier for an instance of this document.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// An identifier, assigned by the sender, for the process file (i.e., record) to which this document
/// belongs.
    #[serde(default, rename = "ContractFolderID")]
    pub contract_folder_id: Option<cct::Identifier>,
/// The date, assigned by the sender, on which this document was issued.
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
/// The time, assigned by the sender, at which this document was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// Short title of a contract associated with this Tender.
    #[serde(default, rename = "ContractName")]
    pub contract_name: Vec<cct::Text>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// A code signifying the type of this tendering procedure.
    #[serde(default, rename = "ProcedureCode")]
    pub procedure_code: Option<cct::Code>,
/// The period during which tenders must be delivered.
    #[serde(default, rename = "TenderSubmissionDeadlinePeriod")]
    pub tender_submission_deadline_period: Option<cac::Period>,
/// The period during which invitations to tender must be completed and delivered.
    #[serde(default, rename = "InvitationSubmissionPeriod")]
    pub invitation_submission_period: Option<cac::Period>,
/// The period during which requests for participation must be completed and delivered.
    #[serde(default, rename = "ParticipationRequestReceptionPeriod")]
    pub participation_request_reception_period: Option<cac::Period>,
/// A reference to a document providing references to procurement legislation applicable to the
/// tendering process.
    #[serde(default, rename = "ProcurementLegislationDocumentReference")]
    pub procurement_legislation_document_reference:
        Option<cac::DocumentReference>,
/// A reference to a document providing references to fiscal legislation applicable to the tendering
/// process.
    #[serde(default, rename = "FiscalLegislationDocumentReference")]
    pub fiscal_legislation_document_reference: Option<cac::DocumentReference>,
/// A reference to a document providing references to environmental legislation applicable to the
/// tendering process.
    #[serde(default, rename = "EnvironmentalLegislationDocumentReference")]
    pub environmental_legislation_document_reference:
        Option<cac::DocumentReference>,
/// A reference to a document providing references to employment legislation applicable to the tendering
/// process.
    #[serde(default, rename = "EmploymentLegislationDocumentReference")]
    pub employment_legislation_document_reference:
        Option<cac::DocumentReference>,
/// A reference to a Procedure Status Request.
    #[serde(default, rename = "ProcedureStatusRequestDocumentReference")]
    pub procedure_status_request_document_reference: Vec<cac::DocumentReference>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The Contracting Party issuing the information about the tender status.
    #[serde(rename = "ContractingParty")]
    pub contracting_party: cac::ContractingParty,
/// The Economic Operator receiving the tender status information.
    #[serde(rename = "EconomicOperatorParty")]
    pub economic_operator_party: cac::EconomicOperatorParty,
/// The Party that provides the procurement documents to the Economic Operator.
    #[serde(default, rename = "DocumentProviderParty")]
    pub document_provider_party: Option<cac::Party>,
/// The Party to which tenders will be submitted.
    #[serde(default, rename = "TenderRecipientParty")]
    pub tender_recipient_party: Option<cac::Party>,
/// An overall definition of this Procurement Project.
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: Option<cac::ProcurementProject>,
/// One of the Procurement Project lots into which this contract can be split.
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: Vec<cac::ProcurementProjectLot>,
}
