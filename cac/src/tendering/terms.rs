#[derive(Debug, Deserialize, Serialize)]
pub struct TenderingTerms {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "AwardingMethodTypeCode")]
    pub awarding_method_type_code: Option<cct::Code>,
    #[serde(default, rename = "PriceEvaluationCode")]
    pub price_evaluation_code: Option<cct::Code>,
    #[serde(default, rename = "MaximumVariantQuantity")]
    pub maximum_variant_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "VariantConstraintIndicator")]
    pub variant_constraint_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "AcceptedVariantsDescription")]
    pub accepted_variants_description: Vec<cct::Text>,
    #[serde(default, rename = "VariantConstraintCode")]
    pub variant_constraint_code: Option<cct::Code>,
    #[serde(default, rename = "PriceRevisionFormulaDescription")]
    pub price_revision_formula_description: Vec<cct::Text>,
    #[serde(default, rename = "FundingProgramCode")]
    pub funding_program_code: Option<cct::Code>,
    #[serde(default, rename = "FundingProgram")]
    pub funding_program: Vec<cct::Text>,
    #[serde(default, rename = "MaximumAdvertisementAmount")]
    pub maximum_advertisement_amount: Option<cct::Amount>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "PaymentFrequencyCode")]
    pub payment_frequency_code: Option<cct::Code>,
    #[serde(default, rename = "EconomicOperatorRegistryURI")]
    pub economic_operator_registry_uri: Option<cct::Identifier>,
    #[serde(default, rename = "RequiredCurriculaIndicator")]
    pub required_curricula_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "RequiredCurriculaCode")]
    pub required_curricula_code: Option<cct::Code>,
    #[serde(default, rename = "OtherConditionsIndicator")]
    pub other_conditions_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "RecurringProcurementIndicator")]
    pub recurring_procurement_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "RecurringProcurementDescription")]
    pub recurring_procurement_description: Vec<cct::Text>,
    #[serde(default, rename = "EstimatedTimingFurtherPublication")]
    pub estimated_timing_further_publication: Vec<cct::Text>,
    #[serde(default, rename = "AdditionalConditions")]
    pub additional_conditions: Vec<cct::Text>,
    #[serde(default, rename = "LatestSecurityClearanceDate")]
    pub latest_security_clearance_date: Option<udt::DateTime>,
    #[serde(default, rename = "DocumentationFeeAmount")]
    pub documentation_fee_amount: Option<cct::Amount>,
    #[serde(default, rename = "MultipleTendersCode")]
    pub multiple_tenders_code: Option<cct::Code>,
    #[serde(default, rename = "PenaltyClause")]
    pub penalty_clause: Vec<crate::Clause>,
    #[serde(default, rename = "RequiredFinancialGuarantee")]
    pub required_financial_guarantee: Vec<crate::FinancialGuarantee>,
    #[serde(default, rename = "ProcurementLegislationDocumentReference")]
    pub procurement_legislation_document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "FiscalLegislationDocumentReference")]
    pub fiscal_legislation_document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "EnvironmentalLegislationDocumentReference")]
    pub environmental_legislation_document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "EmploymentLegislationDocumentReference")]
    pub employment_legislation_document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "ContractualDocumentReference")]
    pub contractual_document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "CallForTendersDocumentReference")]
    pub call_for_tenders_document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "WarrantyValidityPeriod")]
    pub warranty_validity_period: Option<crate::Period>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Vec<crate::PaymentTerms>,
    #[serde(default, rename = "TendererQualificationRequest")]
    pub tenderer_qualification_request: Vec<crate::TendererQualificationRequest>,
    #[serde(default, rename = "AllowedSubcontractTerms")]
    pub allowed_subcontract_terms: Vec<crate::SubcontractTerms>,
    #[serde(default, rename = "TenderPreparation")]
    pub tender_preparation: Vec<crate::TenderPreparation>,
    #[serde(default, rename = "ContractExecutionRequirement")]
    pub contract_execution_requirement: Vec<crate::ContractExecutionRequirement>,
    #[serde(default, rename = "AwardingTerms")]
    pub awarding_terms: Option<crate::AwardingTerms>,
    #[serde(default, rename = "AdditionalInformationParty")]
    pub additional_information_party: Option<crate::Party>,
    #[serde(default, rename = "DocumentProviderParty")]
    pub document_provider_party: Option<crate::Party>,
    #[serde(default, rename = "TenderRecipientParty")]
    pub tender_recipient_party: Option<crate::Party>,
    #[serde(default, rename = "ContractResponsibleParty")]
    pub contract_responsible_party: Option<crate::Party>,
    #[serde(default, rename = "TenderEvaluationParty")]
    pub tender_evaluation_party: Vec<crate::Party>,
    #[serde(default, rename = "QualificationRequestRecipientParty")]
    pub qualification_request_recipient_party: Option<crate::Party>,
    #[serde(default, rename = "TenderValidityPeriod")]
    pub tender_validity_period: Option<crate::Period>,
    #[serde(default, rename = "ContractAcceptancePeriod")]
    pub contract_acceptance_period: Option<crate::Period>,
    #[serde(default, rename = "AppealTerms")]
    pub appeal_terms: Option<crate::AppealTerms>,
    #[serde(default, rename = "Language")]
    pub language: Vec<crate::Language>,
    #[serde(default, rename = "BudgetAccountLine")]
    pub budget_account_line: Vec<crate::BudgetAccountLine>,
    #[serde(default, rename = "ReplacedNoticeDocumentReference")]
    pub replaced_notice_document_reference: Option<crate::DocumentReference>,
    #[serde(default, rename = "LotDistribution")]
    pub lot_distribution: Option<crate::LotDistribution>,
    #[serde(default, rename = "PostAwardProcess")]
    pub post_award_process: Option<crate::PostAwardProcess>,
    #[serde(default, rename = "EconomicOperatorShortList")]
    pub economic_operator_short_list: Option<crate::EconomicOperatorShortList>,
    #[serde(default, rename = "SecurityClearanceTerm")]
    pub security_clearance_term: Vec<crate::SecurityClearanceTerm>,
}
