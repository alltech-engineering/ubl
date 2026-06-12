// UBL Tendering & Procurement CAC types.
//
// These are ABIEs (Aggregate Business Information Entities) used by the
// 22 tendering + 2 quotation document types.
//
// Reference: UBL-CommonAggregateComponents-2.5.xsd

use serde::{Deserialize, Serialize};
use crate::cbc::*;

// ─── UBL Extensions (common to all documents) ─────────────────────

/// UBL Extensions container — standard across all UBL document types.
/// Holds custom extensions for UBL documents.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBLExtensions {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<Extension>,
}

/// A single UBL extension.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Extension {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extension_reason_code: Option<Code>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extension_reason: Option<Text>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extension_agency_id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extension_agency_name: Option<Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extension_content: Option<Text>,
}

// ─── Party wrappers ──────────────────────────────────────────────────
//
// Party roles specific to tendering. Each wraps a Party transparently.

macro_rules! party_wrapper {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name {
            pub party: crate::cac::party::Party,
        }
    };
}

party_wrapper!(TendererParty, "Party submitting a tender.");
party_wrapper!(ContractingParty, "Party issuing the tender / awarding the contract.");
party_wrapper!(EconomicOperatorParty, "Economic operator participating in the procedure.");
party_wrapper!(SubcontractorParty, "Subcontractor party.");
party_wrapper!(OriginatorCustomerParty, "Party originating the document.");
party_wrapper!(BeneficiaryParty, "Beneficiary party.");
party_wrapper!(BuyerCustomerParty, "Buyer / customer party.");
party_wrapper!(SellerSupplierParty, "Seller / supplier party.");
party_wrapper!(SenderParty, "Party sending the document.");
party_wrapper!(ReceiverParty, "Party receiving the document.");
party_wrapper!(DocumentProviderParty, "Party providing access to documents.");
party_wrapper!(TenderRecipientParty, "Party designated to receive tenders.");
party_wrapper!(RequestorParty, "Party making the request.");
party_wrapper!(ResponderParty, "Party responding to the request.");
party_wrapper!(SignatoryParty, "Party signing the document.");

// ─── Document Reference specializations ──────────────────────────────

use crate::cac::document_reference::DocumentReference;

macro_rules! doc_ref_newtype {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub DocumentReference);
    };
}

doc_ref_newtype!(CallForTenderDocumentReference, "Reference to a CallForTenders document.");
doc_ref_newtype!(TenderDocumentReference, "Reference to a Tender document.");
doc_ref_newtype!(TenderNotificationDocumentReference, "Reference to a tender notification.");
doc_ref_newtype!(TenderStatusInquiryDocumentReference, "Reference to a tender status inquiry.");
doc_ref_newtype!(TendererQualificationDocumentReference, "Reference to a qualification document.");
doc_ref_newtype!(ProcurementLegislationDocumentReference, "Reference to procurement legislation.");
doc_ref_newtype!(FiscalLegislationDocumentReference, "Reference to fiscal legislation.");
doc_ref_newtype!(EnvironmentalLegislationDocumentReference, "Reference to environmental legislation.");
doc_ref_newtype!(EmploymentLegislationDocumentReference, "Reference to employment legislation.");
doc_ref_newtype!(LegalDocumentReference, "Reference to a legal document.");
doc_ref_newtype!(TechnicalDocumentReference, "Reference to a technical document.");
doc_ref_newtype!(RequiredDocumentReference, "Reference to a required document.");
doc_ref_newtype!(ProvidedDocumentReference, "Reference to a provided document.");
doc_ref_newtype!(ResolutionDocumentReference, "Reference to a resolution document.");
doc_ref_newtype!(RequestForQuotationDocumentReference, "Reference to a RequestForQuotation.");
doc_ref_newtype!(MinutesDocumentReference, "Reference to meeting minutes.");
doc_ref_newtype!(PreviousDocumentReference, "Reference to a previous document.");
doc_ref_newtype!(AdditionalDocumentReference, "Reference to an additional supporting document.");
doc_ref_newtype!(ParentDocumentReference, "Reference to the parent document.");
doc_ref_newtype!(ExpressionOfInterestDocumentReference, "Reference to an ExpressionOfInterest.");
doc_ref_newtype!(UnsubscribeToProcedureDocumentReference, "Reference to an unsubscribe document.");
doc_ref_newtype!(CatalogueDocumentReference, "Reference to a Catalogue.");
doc_ref_newtype!(ContractDocumentReference, "Reference to a Contract document.");

// ─── Period specializations ──────────────────────────────────────────

use crate::cac::period::Period;

macro_rules! period_newtype {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub Period);
    };
}

period_newtype!(ValidityPeriod, "Period during which something is valid.");
period_newtype!(RequestedValidityPeriod, "Requested validity period.");
period_newtype!(TenderSubmissionDeadlinePeriod, "Deadline for tender submission.");
period_newtype!(InvitationSubmissionPeriod, "Period for submitting invitations.");
period_newtype!(ParticipationRequestReceptionPeriod, "Period for receiving participation requests.");
period_newtype!(FrequencyPeriod, "Frequency period for notices.");
period_newtype!(NominationPeriod, "Nomination period.");

// ─── Procurement ─────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcurementProject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub procurement_type_code: Option<Code>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcurementProjectLot {
    pub id: ID,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<Description>,
}

/// Reference to a ProcurementProjectLot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcurementProjectLotReference {
    pub id: ID,
}

// ─── Tendering Process ───────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TenderingProcess {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub procedure_code: Option<Code>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub urgency_code: Option<Code>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submission_method_code: Option<Code>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub candidate_reduction_constraint_indicator: Option<Indicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub government_agreement_constraint_indicator: Option<Indicator>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TenderingTerms {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<Description>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub award_method_code: Option<Code>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_evaluation_code: Option<Code>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_variant_quantity: Option<Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant_constraint_indicator: Option<Indicator>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accepted_variants_description: Vec<Description>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub price_revision_formula_description: Vec<Description>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub funding_program_code: Option<Code>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub funding_program: Option<Text>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_advertisement_amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_conditions: Option<Text>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_security_clearance_date: Option<Date>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation_fee_amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub penalty_clause: Vec<Text>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_financial_guarantee: Vec<RequiredFinancialGuarantee>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub appeal_terms: Option<AppealTerms>,
}

/// Required financial guarantee.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequiredFinancialGuarantee {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guarantee_type_code: Option<Code>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub liability_amount: Option<Amount>,
}

/// A tendering criterion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TenderingCriterion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub criterion_type_code: Option<Code>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<Description>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight_numeric: Option<Numeric>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold_amount: Option<Amount>,
}

/// Response to a tendering criterion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TenderingCriterionResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<Description>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_value: Option<Text>,
}

/// Result of a tender evaluation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TenderResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tender_result_code: Option<Code>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<Description>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub advertisement_amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub award_date: Option<Date>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub award_time: Option<Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub received_tender_quantity: Option<Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lower_tender_amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub higher_tender_amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Date>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub received_submission: Vec<ReceivedSubmission>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contracted_economic_operator_party: Vec<EconomicOperatorParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract: Option<Contract>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub awarded_tendered_project: Option<AwardedTenderedProject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_formalization_period: Option<ContractFormalizationPeriod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subcontracting_terms: Option<SubcontractingTerms>,
}

/// Tenderer party qualification details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TendererPartyQualification {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interested_procurement_project_lot: Vec<ProcurementProjectLot>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub main_qualifying_party: Vec<QualifyingParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_qualifying_party: Vec<AdditionalQualifyingParty>,
}

/// A qualifying party.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualifyingParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<crate::cac::party::Party>,
}

/// An additional qualifying party.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionalQualifyingParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<crate::cac::party::Party>,
}

/// Qualification resolution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualificationResolution {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution_code: Option<Code>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<Description>,
}

/// A tendered project.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TenderedProject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variant_id: Vec<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tendering_criterion_response: Vec<TenderingCriterionResponse>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
}

/// Awarded tendered project.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwardedTenderedProject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
}

// ─── Regulatory & Notice ─────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegulatoryDomain {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<Name>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<Description>,
}

/// Additional notice language.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionalNoticeLanguage {
    pub id: ID,
    pub language_code: LanguageCode,
}

// ─── Financial ───────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinalFinancialGuarantee {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guarantee_type_code: Option<Code>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub liability_amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_rate: Option<Rate>,
}

// ─── Evidence ────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Evidence {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_type_code: Option<Code>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<Description>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub candidate_statement: Vec<Text>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_issuing_party: Vec<EvidenceIssuingParty>,
}

/// Evidence issuing party.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidenceIssuingParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<crate::cac::party::Party>,
}

// ─── Appeal ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppealTerms {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<Description>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_period: Option<Period>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub appeal_receiver_party: Option<ReceiverParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub appeal_mediator_party: Option<ReceiverParty>,
}

// ─── Quotation ───────────────────────────────────────────────────────

use crate::cac::line_item::LineItem;

/// A line in a RequestForQuotation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestForQuotationLine {
    pub id: ID,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_item: Option<LineItem>,
}

/// A line in a Quotation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuotationLine {
    pub id: ID,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_item: Option<LineItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub seller_proposed_substitute_line_item: Vec<LineItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alternative_line_item: Vec<AlternativeLineItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_for_quotation_line_reference: Option<LineReference>,
}

/// Request for Quotation line reference.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineReference {
    pub id: ID,
}

/// Alternative line item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlternativeLineItem {
    pub id: ID,
}

/// The monetary total for a quotation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuotedMonetaryTotal {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_extension_amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_exclusive_amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_inclusive_amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowance_total_amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub charge_total_amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prepaid_amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payable_rounding_amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payable_amount: Option<Amount>,
}

// ─── Signature (tendering-specific) ──────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Signature {
    pub id: ID,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation_date: Option<Date>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation_time: Option<Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validator_id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canonicalization_method: Option<Text>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_method: Option<Text>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signatory_party: Option<SignatoryParty>,
}

// ─── Received Submission ────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReceivedSubmission {
    pub id: ID,
}

// ─── Contract (tendering-specific) ──────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contract {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<Date>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_type_code: Option<Code>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<Text>,
}

// ─── Misc support types ──────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ContractFormalizationPeriod(pub Period);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubcontractingTerms {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<Rate>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<Description>,
}

/// Destination country (used in quotation).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinationCountry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identification_code: Option<CountryCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<CountryName>,
}

/// Requested tender total.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestedTenderTotal {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated_amount: Option<Amount>,
}

/// Budget account line.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BudgetAccountLine {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
}
