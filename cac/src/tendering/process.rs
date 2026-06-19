#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the process of a formal offer and response to execute work or supply goods at a
/// stated price.
///
/// UBL Dictionary Entry Name: `Tendering Process. Details`
///
/// Generated from XSD type `TenderingProcessType`.
pub struct TenderingProcess {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this tendering process.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// When reopening a tendering process, the identifier of the original framework agreement or dynamic
/// purchasing system.
    #[serde(default, rename = "OriginalContractingSystemID")]
    pub original_contracting_system_id: Option<cct::Identifier>,
/// Text describing the tendering process.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// Text describing the negotiation to be followed during the tendering process.
    #[serde(default, rename = "NegotiationDescription")]
    pub negotiation_description: Vec<cct::Text>,
/// A code signifying the type of this tendering procedure.
    #[serde(default, rename = "ProcedureCode")]
    pub procedure_code: Option<cct::Code>,
/// A code signifying the urgency of this tendering process.
    #[serde(default, rename = "UrgencyCode")]
    pub urgency_code: Option<cct::Code>,
/// A code signifying the type of expense for this tendering process.
    #[serde(default, rename = "ExpenseCode")]
    pub expense_code: Option<cct::Code>,
/// A code signifying the type of presentation of tenders required (e.g., one lot, multiple lots, or all
/// the lots).
    #[serde(default, rename = "PartPresentationCode")]
    pub part_presentation_code: Option<cct::Code>,
/// A code signifying the type of contracting system (e.g., framework agreement, dynamic purchasing
/// system). If the procedure is individual (nonrepetitive), this code ought to be omitted.
    #[serde(default, rename = "ContractingSystemCode")]
    pub contracting_system_code: Option<cct::Code>,
/// A code signifying the method to be followed in submitting tenders.
    #[serde(default, rename = "SubmissionMethodCode")]
    pub submission_method_code: Option<cct::Code>,
/// An indicator that the number of candidates participating in this process has been reduced (true) or
/// not (false).
    #[serde(default, rename = "CandidateReductionConstraintIndicator")]
    pub candidate_reduction_constraint_indicator: Option<udt::Indicator>,
/// An indicator that the project associated with this tendering process is constrained by a government
/// procurement agreement (true) or not (false).
    #[serde(default, rename = "GovernmentAgreementConstraintIndicator")]
    pub government_agreement_constraint_indicator: Option<udt::Indicator>,
/// The URI where the tools for electronic communication related with the tendering process can be
/// found.
    #[serde(default, rename = "AccessToolsURI")]
    pub access_tools_uri: Option<cct::Identifier>,
/// An indicator that the competition launched is terminated.
    #[serde(default, rename = "TerminatedIndicator")]
    pub terminated_indicator: Option<udt::Indicator>,
/// The period during which documents relating to this tendering process must be completed.
    #[serde(default, rename = "DocumentAvailabilityPeriod")]
    pub document_availability_period: Option<crate::Period>,
/// The period during which tenders must be delivered.
    #[serde(default, rename = "TenderSubmissionDeadlinePeriod")]
    pub tender_submission_deadline_period: Option<crate::Period>,
/// The period during which invitations to tender must be completed and delivered.
    #[serde(default, rename = "InvitationSubmissionPeriod")]
    pub invitation_submission_period: Option<crate::Period>,
/// The period during which the invitation to participate must be sent.
    #[serde(default, rename = "ParticipationInvitationPeriod")]
    pub participation_invitation_period: Option<crate::Period>,
/// The period during which requests for participation must be completed and delivered.
    #[serde(default, rename = "ParticipationRequestReceptionPeriod")]
    pub participation_request_reception_period: Option<crate::Period>,
/// The period during which additional information about the procurement can be requested.
    #[serde(default, rename = "AdditionalInformationRequestPeriod")]
    pub additional_information_request_period: Option<crate::Period>,
/// A reference to a notice pertaining to this tendering process.
    #[serde(default, rename = "NoticeDocumentReference")]
    pub notice_document_reference: Vec<crate::DocumentReference>,
/// A reference to an additional document.
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<crate::DocumentReference>,
/// A justification for the selection of this tendering process.
    #[serde(default, rename = "ProcessJustification")]
    pub process_justification: Vec<crate::ProcessJustification>,
/// A set of criteria used to create a short list of candidates.
    #[serde(default, rename = "EconomicOperatorShortList")]
    pub economic_operator_short_list: Vec<crate::EconomicOperatorShortList>,
/// An Event specifying the location and time of the public opening of tenders.
    #[serde(default, rename = "OpenTenderEvent")]
    pub open_tender_event: Vec<crate::Event>,
/// The terms to be fulfilled by tenderers if an auction is to be executed before the awarding of a
/// tender.
    #[serde(default, rename = "AuctionTerms")]
    pub auction_terms: Option<crate::AuctionTerms>,
/// A tendering framework agreement.
    #[serde(default, rename = "FrameworkAgreement")]
    pub framework_agreement: Option<crate::FrameworkAgreement>,
/// A reference to a contracting system. Only when the procedure is repetitive.
    #[serde(default, rename = "ContractingSystem")]
    pub contracting_system: Vec<crate::ContractingSystem>,
}
