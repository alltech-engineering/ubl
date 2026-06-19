#[derive(Debug, Deserialize, Serialize)]
/// A class to describe tendering terms for a tendering process.
///
/// UBL Dictionary Entry Name: `Tendering Terms. Details`
///
/// Generated from XSD type `TenderingTermsType`.
pub struct TenderingTerms {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code signifying the awarding method in a tendering process (e.g., a method favoring the tender
/// with the lowest price or the tender that is most economically advantageous).
    #[serde(default, rename = "AwardingMethodTypeCode")]
    pub awarding_method_type_code: Option<cct::Code>,
/// Textual description of the legal form required for potential tenderers.
    #[serde(default, rename = "PriceEvaluationCode")]
    pub price_evaluation_code: Option<cct::Code>,
/// Maximum number of variants the tenderer is allowed to present for this tendering project.
    #[serde(default, rename = "MaximumVariantQuantity")]
    pub maximum_variant_quantity: Option<cct::Quantity>,
/// An indicator that variants are allowed and unconstrained in number (true) or not allowed (false).
    #[serde(default, rename = "VariantConstraintIndicator")]
    pub variant_constraint_indicator: Option<udt::Indicator>,
/// Text specifying the things for which variants are accepted.
    #[serde(default, rename = "AcceptedVariantsDescription")]
    pub accepted_variants_description: Vec<cct::Text>,
/// A code signifying the modalities for a tenderer to submit variants of tenders.
    #[serde(default, rename = "VariantConstraintCode")]
    pub variant_constraint_code: Option<cct::Code>,
/// Text describing the formula for price revision.
    #[serde(default, rename = "PriceRevisionFormulaDescription")]
    pub price_revision_formula_description: Vec<cct::Text>,
/// The program that funds the tendering process (e.g., "National", "European"), expressed as a code.
    #[serde(default, rename = "FundingProgramCode")]
    pub funding_program_code: Option<cct::Code>,
/// The program that funds the tendering process (e.g., EU 6th Framework Program) expressed as text.
    #[serde(default, rename = "FundingProgram")]
    pub funding_program: Vec<cct::Text>,
/// The maximum advertised monetary value of the tendering process.
    #[serde(default, rename = "MaximumAdvertisementAmount")]
    pub maximum_advertisement_amount: Option<cct::Amount>,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// A code signifying the frequency of payment in the contract associated with the tendering process.
    #[serde(default, rename = "PaymentFrequencyCode")]
    pub payment_frequency_code: Option<cct::Code>,
/// The Uniform Resource Identifier (URI) of an electronic registry of economic operators.
    #[serde(default, rename = "EconomicOperatorRegistryURI")]
    pub economic_operator_registry_uri: Option<cct::Identifier>,
/// An indicator that tenderers are required to provide a curriculum vitae for each participant in the
/// project (true) or are not so required (false).
    #[serde(default, rename = "RequiredCurriculaIndicator")]
    pub required_curricula_indicator: Option<udt::Indicator>,
/// A code signifying the conditions applying for tenderers to provide a curriculum vitae.
    #[serde(default, rename = "RequiredCurriculaCode")]
    pub required_curricula_code: Option<cct::Code>,
/// Indicates whether other conditions exist (true) or not (false). If the indicator is true, the
/// description may be provided.
    #[serde(default, rename = "OtherConditionsIndicator")]
    pub other_conditions_indicator: Option<udt::Indicator>,
/// Indicates whether the procurement is recurring (true) or not (false).
    #[serde(default, rename = "RecurringProcurementIndicator")]
    pub recurring_procurement_indicator: Option<udt::Indicator>,
/// Any additional information about recurrence (e.g. estimated timing).
    #[serde(default, rename = "RecurringProcurementDescription")]
    pub recurring_procurement_description: Vec<cct::Text>,
/// The description of the estimated timing for further notices to be published.
    #[serde(default, rename = "EstimatedTimingFurtherPublication")]
    pub estimated_timing_further_publication: Vec<cct::Text>,
/// Other existing conditions.
    #[serde(default, rename = "AdditionalConditions")]
    pub additional_conditions: Vec<cct::Text>,
/// The end date until which the candidates can obtain the necessary level of security clearance.
    #[serde(default, rename = "LatestSecurityClearanceDate")]
    pub latest_security_clearance_date: Option<udt::DateTime>,
/// The amount to be paid to obtain the contract documents and additional documentation.
    #[serde(default, rename = "DocumentationFeeAmount")]
    pub documentation_fee_amount: Option<cct::Amount>,
/// A code signifying whether a tenderer is allowed to submit multiple tenders.
    #[serde(default, rename = "MultipleTendersCode")]
    pub multiple_tenders_code: Option<cct::Code>,
/// The penalty clauses
    #[serde(default, rename = "PenaltyClause")]
    pub penalty_clause: Vec<crate::Clause>,
/// A financial guarantee of a tenderer or bid submitter's actual entry into a contract in the event
/// that it is the successful bidder.
    #[serde(default, rename = "RequiredFinancialGuarantee")]
    pub required_financial_guarantee: Vec<crate::FinancialGuarantee>,
/// A reference to a document providing references to procurement legislation applicable to the
/// tendering process.
    #[serde(default, rename = "ProcurementLegislationDocumentReference")]
    pub procurement_legislation_document_reference: Vec<crate::DocumentReference>,
/// A reference to a document providing references to fiscal legislation applicable to the tendering
/// process.
    #[serde(default, rename = "FiscalLegislationDocumentReference")]
    pub fiscal_legislation_document_reference: Vec<crate::DocumentReference>,
/// A reference to a document providing references to environmental legislation applicable to the
/// tendering process.
    #[serde(default, rename = "EnvironmentalLegislationDocumentReference")]
    pub environmental_legislation_document_reference: Vec<crate::DocumentReference>,
/// A reference to a document providing references to employment legislation applicable to the tendering
/// process.
    #[serde(default, rename = "EmploymentLegislationDocumentReference")]
    pub employment_legislation_document_reference: Vec<crate::DocumentReference>,
/// A reference to a document that will become part of the awarded contract.
    #[serde(default, rename = "ContractualDocumentReference")]
    pub contractual_document_reference: Vec<crate::DocumentReference>,
/// A reference to a Call for Tender associated with these tendering terms.
    #[serde(default, rename = "CallForTendersDocumentReference")]
    pub call_for_tenders_document_reference: Vec<crate::DocumentReference>,
/// The period during which a warranty for work, service, or goods associated with these tendering terms
/// is valid.
    #[serde(default, rename = "WarrantyValidityPeriod")]
    pub warranty_validity_period: Option<crate::Period>,
/// A specification of payment terms associated with the tendering process.
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Vec<crate::PaymentTerms>,
/// Required set of qualifications for a tenderer in this tendering process.
    #[serde(default, rename = "TendererQualificationRequest")]
    pub tenderer_qualification_request: Vec<crate::TendererQualificationRequest>,
/// Subcontract terms for the tendering process.
    #[serde(default, rename = "AllowedSubcontractTerms")]
    pub allowed_subcontract_terms: Vec<crate::SubcontractTerms>,
/// Directions for preparing a tender for the+D2057 tendering process.
    #[serde(default, rename = "TenderPreparation")]
    pub tender_preparation: Vec<crate::TenderPreparation>,
/// A requirement relating to execution of the contract that will be awarded as a result of the
/// tendering process.
    #[serde(default, rename = "ContractExecutionRequirement")]
    pub contract_execution_requirement: Vec<crate::ContractExecutionRequirement>,
/// The terms in the tendering process for awarding the contract for a project.
    #[serde(default, rename = "AwardingTerms")]
    pub awarding_terms: Option<crate::AwardingTerms>,
/// The Party who has additional information about the tendering process.
    #[serde(default, rename = "AdditionalInformationParty")]
    pub additional_information_party: Option<crate::Party>,
/// The Party who has the Contract Documents for the tendering process.
    #[serde(default, rename = "DocumentProviderParty")]
    pub document_provider_party: Option<crate::Party>,
/// The Party who receives the Tenders.
    #[serde(default, rename = "TenderRecipientParty")]
    pub tender_recipient_party: Option<crate::Party>,
/// The Party who executes the Contract.
    #[serde(default, rename = "ContractResponsibleParty")]
    pub contract_responsible_party: Option<crate::Party>,
/// The Buyer Party who evaluates the Tenders received.
    #[serde(default, rename = "TenderEvaluationParty")]
    pub tender_evaluation_party: Vec<crate::Party>,
/// The Buyer Party who receives the Qualification Request.
    #[serde(default, rename = "QualificationRequestRecipientParty")]
    pub qualification_request_recipient_party: Option<crate::Party>,
/// The period during which tenders submitted for this tendering process must remain valid.
    #[serde(default, rename = "TenderValidityPeriod")]
    pub tender_validity_period: Option<crate::Period>,
/// The period of time during which the contracting authority may accept a contract.
    #[serde(default, rename = "ContractAcceptancePeriod")]
    pub contract_acceptance_period: Option<crate::Period>,
/// Information about the terms to present for an appeal against a tender award.
    #[serde(default, rename = "AppealTerms")]
    pub appeal_terms: Option<crate::AppealTerms>,
/// One of the default languages specified for the tendering process.
    #[serde(default, rename = "Language")]
    pub language: Vec<crate::Language>,
/// A budget account line associated with the tendering process.
    #[serde(default, rename = "BudgetAccountLine")]
    pub budget_account_line: Vec<crate::BudgetAccountLine>,
/// A class defining a reference to the notice that is being replaced.
    #[serde(default, rename = "ReplacedNoticeDocumentReference")]
    pub replaced_notice_document_reference: Option<crate::DocumentReference>,
/// List of specific ways to tender to the lots of the procurement project.
    #[serde(default, rename = "LotDistribution")]
    pub lot_distribution: Option<crate::LotDistribution>,
/// Information about the post-award process.
    #[serde(default, rename = "PostAwardProcess")]
    pub post_award_process: Option<crate::PostAwardProcess>,
/// A set of criteria used to create a short list of candidates.
    #[serde(default, rename = "EconomicOperatorShortList")]
    pub economic_operator_short_list: Option<crate::EconomicOperatorShortList>,
/// Information about the terms to present for a security clearance.
    #[serde(default, rename = "SecurityClearanceTerm")]
    pub security_clearance_term: Vec<crate::SecurityClearanceTerm>,
}
