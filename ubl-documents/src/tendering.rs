// UBL 2.5 Tendering document types.
//
// Reference: https://docs.oasis-open.org/ubl/cs01-UBL-2.5/UBL-2.5.html
// Generated from the authoritative XSD element declarations.

use serde::{Deserialize, Serialize};
use ubl_common::cbc::*;
use ubl_common::cac::tendering::*;
use ubl_common::cac::*;

/// Tender — UBL 2.5 document type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tender {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_extensions: Option<UBLExtensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    pub id: ID,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tender_type_code: Option<TenderTypeCode>,
    pub contract_folder_id: ContractFolderID,
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contract_name: Vec<ContractName>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<ValidityPeriod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub call_for_tender_document_reference: Option<CallForTenderDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tenderer_party: Vec<TendererParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenderer_qualification_document_reference: Option<TendererQualificationDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subcontractor_party: Vec<SubcontractorParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contracting_party: Vec<ContractingParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originator_customer_party: Option<OriginatorCustomerParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub beneficiary_party: Vec<BeneficiaryParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tendered_project: Vec<TenderedProject>,
}

/// TenderReceipt — UBL 2.5 document type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TenderReceipt {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_extensions: Option<UBLExtensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    pub contract_folder_id: ContractFolderID,
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contract_name: Vec<ContractName>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    pub registered_date: RegisteredDate,
    pub registered_time: RegisteredTime,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tender_document_reference: Vec<TenderDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
    pub sender_party: SenderParty,
    pub receiver_party: ReceiverParty,
}

/// TenderStatus — UBL 2.5 document type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TenderStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_extensions: Option<UBLExtensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_folder_id: Option<ContractFolderID>,
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contract_name: Vec<ContractName>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub procedure_code: Option<ProcedureCode>,
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
    pub environmental_legislation_document_reference: Option<EnvironmentalLegislationDocumentReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_legislation_document_reference: Option<EmploymentLegislationDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tender_status_inquiry_document_reference: Vec<TenderStatusInquiryDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
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

/// TenderStatusRequest — UBL 2.5 document type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TenderStatusRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_extensions: Option<UBLExtensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_folder_id: Option<ContractFolderID>,
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contract_name: Vec<ContractName>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contracting_party: Vec<ContractingParty>,
    pub economic_operator_party: EconomicOperatorParty,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub procurement_project: Option<ProcurementProject>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procurement_project_lot: Vec<ProcurementProjectLot>,
}

/// TenderWithdrawal — UBL 2.5 document type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TenderWithdrawal {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_extensions: Option<UBLExtensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_folder_id: Option<ContractFolderID>,
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contract_name: Vec<ContractName>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub withdraw_offer_indicator: Option<WithdrawOfferIndicator>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tender_document_reference: Vec<TenderDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tender_notification_document_reference: Vec<TenderNotificationDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contracting_party: Vec<ContractingParty>,
    pub tenderer_party: TendererParty,
}

/// TendererQualification — UBL 2.5 document type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TendererQualification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_extensions: Option<UBLExtensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    pub id: ID,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    pub contract_folder_id: ContractFolderID,
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<VersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_version_id: Option<PreviousVersionID>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tenderer_party_qualification: Vec<TendererPartyQualification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contracting_party: Option<ContractingParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<Evidence>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_document_reference: Vec<AdditionalDocumentReference>,
}

/// TendererQualificationResponse — UBL 2.5 document type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TendererQualificationResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_extensions: Option<UBLExtensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    pub contract_folder_id: ContractFolderID,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contract_name: Vec<ContractName>,
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    pub sender_party: SenderParty,
    pub receiver_party: ReceiverParty,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution_document_reference: Option<ResolutionDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualification_resolution: Vec<QualificationResolution>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub appeal_terms: Option<AppealTerms>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
}

/// TenderContract — UBL 2.5 document type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TenderContract {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_extensions: Option<UBLExtensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    pub contract_folder_id: ContractFolderID,
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contract_name: Vec<ContractName>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub regulatory_domain: Vec<RegulatoryDomain>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_award_indicator: Option<PublishAwardIndicator>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub previous_document_reference: Vec<PreviousDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contract_document_reference: Vec<ContractDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contracting_party: Vec<ContractingParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub economic_operator_party: Vec<EconomicOperatorParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver_party: Option<ReceiverParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tendering_terms: Option<TenderingTerms>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tendering_process: Option<TenderingProcess>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub procurement_project: Option<ProcurementProject>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procurement_project_lot: Vec<ProcurementProjectLot>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tender_result: Vec<TenderResult>,
}

/// AwardedNotification — UBL 2.5 document type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwardedNotification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_extensions: Option<UBLExtensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    pub contract_folder_id: ContractFolderID,
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contract_name: Vec<ContractName>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    pub sender_party: SenderParty,
    pub receiver_party: ReceiverParty,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minutes_document_reference: Option<MinutesDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_document_reference: Vec<AdditionalDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tender_result: Vec<TenderResult>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub final_financial_guarantee: Vec<FinalFinancialGuarantee>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
}

/// UnawardedNotification — UBL 2.5 document type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnawardedNotification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_extensions: Option<UBLExtensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    pub contract_folder_id: ContractFolderID,
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contract_name: Vec<ContractName>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
    pub sender_party: SenderParty,
    pub receiver_party: ReceiverParty,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minutes_document_reference: Option<MinutesDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_document_reference: Vec<AdditionalDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tender_result: Vec<TenderResult>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub appeal_terms: Option<AppealTerms>,
}

/// CallForTenders — UBL 2.5 document type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallForTenders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_extensions: Option<UBLExtensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    pub contract_folder_id: ContractFolderID,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_date: Option<ApprovalDate>,
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<VersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_version_id: Option<PreviousVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_document_reference: Option<LegalDocumentReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub technical_document_reference: Option<TechnicalDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_document_reference: Vec<RequiredDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provided_document_reference: Vec<ProvidedDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_document_reference: Vec<AdditionalDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contracting_party: Vec<ContractingParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub originator_customer_party: Vec<OriginatorCustomerParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub beneficiary_party: Vec<BeneficiaryParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver_party: Option<ReceiverParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tendering_terms: Option<TenderingTerms>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tendering_process: Option<TenderingProcess>,
    pub procurement_project: ProcurementProject,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procurement_project_lot: Vec<ProcurementProjectLot>,
}

/// ContractNotice — UBL 2.5 document type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractNotice {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_extensions: Option<UBLExtensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    pub contract_folder_id: ContractFolderID,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<IssueDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<VersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_version_id: Option<PreviousVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_publication_date: Option<RequestedPublicationDate>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub regulatory_domain: Vec<RegulatoryDomain>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notice_type_code: Option<NoticeTypeCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notice_language_code: Option<NoticeLanguageCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_notice_language: Vec<AdditionalNoticeLanguage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency_period: Option<FrequencyPeriod>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contracting_party: Vec<ContractingParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub originator_customer_party: Vec<OriginatorCustomerParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub beneficiary_party: Vec<BeneficiaryParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver_party: Option<ReceiverParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tendering_terms: Option<TenderingTerms>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tendering_process: Option<TenderingProcess>,
    pub procurement_project: ProcurementProject,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procurement_project_lot: Vec<ProcurementProjectLot>,
}

/// ContractAwardNotice — UBL 2.5 document type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractAwardNotice {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_extensions: Option<UBLExtensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    pub contract_folder_id: ContractFolderID,
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<VersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_version_id: Option<PreviousVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_publication_date: Option<RequestedPublicationDate>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub regulatory_domain: Vec<RegulatoryDomain>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notice_type_code: Option<NoticeTypeCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_award_indicator: Option<PublishAwardIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notice_language_code: Option<NoticeLanguageCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_notice_language: Vec<AdditionalNoticeLanguage>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub previous_document_reference: Vec<PreviousDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub minutes_document_reference: Vec<MinutesDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contracting_party: Vec<ContractingParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originator_customer_party: Option<OriginatorCustomerParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub beneficiary_party: Vec<BeneficiaryParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver_party: Option<ReceiverParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tendering_terms: Option<TenderingTerms>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tendering_process: Option<TenderingProcess>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub procurement_project: Option<ProcurementProject>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procurement_project_lot: Vec<ProcurementProjectLot>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tender_result: Vec<TenderResult>,
}

/// PriorInformationNotice — UBL 2.5 document type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriorInformationNotice {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_extensions: Option<UBLExtensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_folder_id: Option<ContractFolderID>,
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<VersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_version_id: Option<PreviousVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_publication_date: Option<RequestedPublicationDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub planned_date: Option<PlannedDate>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub regulatory_domain: Vec<RegulatoryDomain>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notice_type_code: Option<NoticeTypeCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notice_language_code: Option<NoticeLanguageCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_notice_language: Vec<AdditionalNoticeLanguage>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contracting_party: Vec<ContractingParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub originator_customer_party: Vec<OriginatorCustomerParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub beneficiary_party: Vec<BeneficiaryParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver_party: Option<ReceiverParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tendering_terms: Option<TenderingTerms>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tendering_process: Option<TenderingProcess>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub procurement_project: Option<ProcurementProject>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procurement_project_lot: Vec<ProcurementProjectLot>,
}

/// ExpressionOfInterestRequest — UBL 2.5 document type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpressionOfInterestRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_extensions: Option<UBLExtensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    pub id: ID,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_folder_id: Option<ContractFolderID>,
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contract_name: Vec<ContractName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_language_locale_code: Option<PreferredLanguageLocaleCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<ValidityPeriod>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
    pub economic_operator_party: EconomicOperatorParty,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contracting_party: Vec<ContractingParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub procurement_project: Option<ProcurementProject>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procurement_project_lot_reference: Vec<ProcurementProjectLotReference>,
}

/// ExpressionOfInterestResponse — UBL 2.5 document type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpressionOfInterestResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_extensions: Option<UBLExtensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    pub id: ID,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_folder_id: Option<ContractFolderID>,
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contract_name: Vec<ContractName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tender_language_locale_code: Option<TenderLanguageLocaleCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub expression_of_interest_document_reference: Vec<ExpressionOfInterestDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
    pub economic_operator_party: EconomicOperatorParty,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contracting_party: Vec<ContractingParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub procurement_project: Option<ProcurementProject>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procurement_project_lot_reference: Vec<ProcurementProjectLotReference>,
}

/// QualificationApplicationRequest — UBL 2.5 document type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualificationApplicationRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_extensions: Option<UBLExtensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    pub contract_folder_id: ContractFolderID,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contract_name: Vec<ContractName>,
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<VersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_version_id: Option<PreviousVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub procedure_code: Option<ProcedureCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qualification_application_type_code: Option<QualificationApplicationTypeCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub weight_scoring_methodology_note: Vec<WeightScoringMethodologyNote>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weighting_type_code: Option<WeightingTypeCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contracting_party: Vec<ContractingParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub economic_operator_party: Vec<EconomicOperatorParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub procurement_project: Option<ProcurementProject>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procurement_project_lot: Vec<ProcurementProjectLot>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tendering_criterion: Vec<TenderingCriterion>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_document_reference: Vec<AdditionalDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
}

/// QualificationApplicationResponse — UBL 2.5 document type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualificationApplicationResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_extensions: Option<UBLExtensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    pub contract_folder_id: ContractFolderID,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contract_name: Vec<ContractName>,
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub economic_operator_group_name: Option<EconomicOperatorGroupName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<VersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_version_id: Option<PreviousVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub procedure_code: Option<ProcedureCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qualification_application_type_code: Option<QualificationApplicationTypeCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub weight_scoring_methodology_note: Vec<WeightScoringMethodologyNote>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weighting_type_code: Option<WeightingTypeCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contracting_party: Vec<ContractingParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub economic_operator_party: Vec<EconomicOperatorParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub procurement_project: Option<ProcurementProject>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procurement_project_lot: Vec<ProcurementProjectLot>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tendering_criterion: Vec<TenderingCriterion>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tendering_criterion_response: Vec<TenderingCriterionResponse>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_document_reference: Vec<AdditionalDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<Evidence>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
}

/// UnsubscribeFromProcedureRequest — UBL 2.5 document type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnsubscribeFromProcedureRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_extensions: Option<UBLExtensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    pub id: ID,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_folder_id: Option<ContractFolderID>,
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
    pub economic_operator_party: EconomicOperatorParty,
    pub contracting_party: ContractingParty,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub procurement_project: Option<ProcurementProject>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procurement_project_lot_reference: Vec<ProcurementProjectLotReference>,
}

/// UnsubscribeFromProcedureResponse — UBL 2.5 document type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnsubscribeFromProcedureResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_extensions: Option<UBLExtensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    pub id: ID,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_folder_id: Option<ContractFolderID>,
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unsubscribe_to_procedure_document_reference: Option<UnsubscribeToProcedureDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
    pub economic_operator_party: EconomicOperatorParty,
    pub contracting_party: ContractingParty,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub procurement_project: Option<ProcurementProject>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procurement_project_lot_reference: Vec<ProcurementProjectLotReference>,
}

/// Enquiry — UBL 2.5 document type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enquiry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_extensions: Option<UBLExtensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_reply_date: Option<LatestReplyDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_reply_time: Option<LatestReplyTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<Description>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
    pub requestor_party: RequestorParty,
    pub responder_party: ResponderParty,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_document_reference: Vec<AdditionalDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachment: Vec<Attachment>,
}

/// EnquiryResponse — UBL 2.5 document type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnquiryResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_extensions: Option<UBLExtensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<Description>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
    pub requestor_party: RequestorParty,
    pub responder_party: ResponderParty,
    pub parent_document_reference: ParentDocumentReference,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_document_reference: Vec<AdditionalDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachment: Vec<Attachment>,
}

