#[derive(Debug, Deserialize, Serialize)]
pub struct TenderingProcess {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "OriginalContractingSystemID")]
    pub original_contracting_system_id: Option<cct::Identifier>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "NegotiationDescription")]
    pub negotiation_description: Vec<cct::Text>,
    #[serde(default, rename = "ProcedureCode")]
    pub procedure_code: Option<cct::Code>,
    #[serde(default, rename = "UrgencyCode")]
    pub urgency_code: Option<cct::Code>,
    #[serde(default, rename = "ExpenseCode")]
    pub expense_code: Option<cct::Code>,
    #[serde(default, rename = "PartPresentationCode")]
    pub part_presentation_code: Option<cct::Code>,
    #[serde(default, rename = "ContractingSystemCode")]
    pub contracting_system_code: Option<cct::Code>,
    #[serde(default, rename = "SubmissionMethodCode")]
    pub submission_method_code: Option<cct::Code>,
    #[serde(default, rename = "CandidateReductionConstraintIndicator")]
    pub candidate_reduction_constraint_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "GovernmentAgreementConstraintIndicator")]
    pub government_agreement_constraint_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "AccessToolsURI")]
    pub access_tools_uri: Option<cct::Identifier>,
    #[serde(default, rename = "TerminatedIndicator")]
    pub terminated_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "DocumentAvailabilityPeriod")]
    pub document_availability_period: Option<crate::Period>,
    #[serde(default, rename = "TenderSubmissionDeadlinePeriod")]
    pub tender_submission_deadline_period: Option<crate::Period>,
    #[serde(default, rename = "InvitationSubmissionPeriod")]
    pub invitation_submission_period: Option<crate::Period>,
    #[serde(default, rename = "ParticipationInvitationPeriod")]
    pub participation_invitation_period: Option<crate::Period>,
    #[serde(default, rename = "ParticipationRequestReceptionPeriod")]
    pub participation_request_reception_period: Option<crate::Period>,
    #[serde(default, rename = "AdditionalInformationRequestPeriod")]
    pub additional_information_request_period: Option<crate::Period>,
    #[serde(default, rename = "NoticeDocumentReference")]
    pub notice_document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "ProcessJustification")]
    pub process_justification: Vec<crate::ProcessJustification>,
    #[serde(default, rename = "EconomicOperatorShortList")]
    pub economic_operator_short_list: Vec<crate::EconomicOperatorShortList>,
    #[serde(default, rename = "OpenTenderEvent")]
    pub open_tender_event: Vec<crate::Event>,
    #[serde(default, rename = "AuctionTerms")]
    pub auction_terms: Option<crate::AuctionTerms>,
    #[serde(default, rename = "FrameworkAgreement")]
    pub framework_agreement: Option<crate::FrameworkAgreement>,
    #[serde(default, rename = "ContractingSystem")]
    pub contracting_system: Vec<crate::ContractingSystem>,
}
