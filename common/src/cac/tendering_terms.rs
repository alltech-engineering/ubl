#[derive(Debug, Deserialize, Serialize)]
pub struct TenderingTerms {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "AwardingMethodTypeCode")]
    pub awarding_method_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "PriceEvaluationCode")]
    pub price_evaluation_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "MaximumVariantQuantity")]
    pub maximum_variant_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "VariantConstraintIndicator")]
    pub variant_constraint_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "AcceptedVariantsDescription")]
    pub accepted_variants_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "VariantConstraintCode")]
    pub variant_constraint_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "PriceRevisionFormulaDescription")]
    pub price_revision_formula_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "FundingProgramCode")]
    pub funding_program_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "FundingProgram")]
    pub funding_program: Vec<super::cct::TextType>,
    #[serde(default, rename = "MaximumAdvertisementAmount")]
    pub maximum_advertisement_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "PaymentFrequencyCode")]
    pub payment_frequency_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "EconomicOperatorRegistryURI")]
    pub economic_operator_registry_uri: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "RequiredCurriculaIndicator")]
    pub required_curricula_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "RequiredCurriculaCode")]
    pub required_curricula_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "OtherConditionsIndicator")]
    pub other_conditions_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "RecurringProcurementIndicator")]
    pub recurring_procurement_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "RecurringProcurementDescription")]
    pub recurring_procurement_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "EstimatedTimingFurtherPublication")]
    pub estimated_timing_further_publication: Vec<super::cct::TextType>,
    #[serde(default, rename = "AdditionalConditions")]
    pub additional_conditions: Vec<super::cct::TextType>,
    #[serde(default, rename = "LatestSecurityClearanceDate")]
    pub latest_security_clearance_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "DocumentationFeeAmount")]
    pub documentation_fee_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "MultipleTendersCode")]
    pub multiple_tenders_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "PenaltyClause")]
    pub penalty_clause: Vec<Clause>,
    #[serde(default, rename = "RequiredFinancialGuarantee")]
    pub required_financial_guarantee: Vec<FinancialGuarantee>,
    #[serde(default, rename = "ProcurementLegislationDocumentReference")]
    pub procurement_legislation_document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "FiscalLegislationDocumentReference")]
    pub fiscal_legislation_document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "EnvironmentalLegislationDocumentReference")]
    pub environmental_legislation_document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "EmploymentLegislationDocumentReference")]
    pub employment_legislation_document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "ContractualDocumentReference")]
    pub contractual_document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "CallForTendersDocumentReference")]
    pub call_for_tenders_document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "WarrantyValidityPeriod")]
    pub warranty_validity_period: Option<Period>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Vec<PaymentTerms>,
    #[serde(default, rename = "TendererQualificationRequest")]
    pub tenderer_qualification_request: Vec<TendererQualificationRequest>,
    #[serde(default, rename = "AllowedSubcontractTerms")]
    pub allowed_subcontract_terms: Vec<SubcontractTerms>,
    #[serde(default, rename = "TenderPreparation")]
    pub tender_preparation: Vec<TenderPreparation>,
    #[serde(default, rename = "ContractExecutionRequirement")]
    pub contract_execution_requirement: Vec<ContractExecutionRequirement>,
    #[serde(default, rename = "AwardingTerms")]
    pub awarding_terms: Option<AwardingTerms>,
    #[serde(default, rename = "AdditionalInformationParty")]
    pub additional_information_party: Option<Party>,
    #[serde(default, rename = "DocumentProviderParty")]
    pub document_provider_party: Option<Party>,
    #[serde(default, rename = "TenderRecipientParty")]
    pub tender_recipient_party: Option<Party>,
    #[serde(default, rename = "ContractResponsibleParty")]
    pub contract_responsible_party: Option<Party>,
    #[serde(default, rename = "TenderEvaluationParty")]
    pub tender_evaluation_party: Vec<Party>,
    #[serde(default, rename = "QualificationRequestRecipientParty")]
    pub qualification_request_recipient_party: Option<Party>,
    #[serde(default, rename = "TenderValidityPeriod")]
    pub tender_validity_period: Option<Period>,
    #[serde(default, rename = "ContractAcceptancePeriod")]
    pub contract_acceptance_period: Option<Period>,
    #[serde(default, rename = "AppealTerms")]
    pub appeal_terms: Option<AppealTerms>,
    #[serde(default, rename = "Language")]
    pub language: Vec<Language>,
    #[serde(default, rename = "BudgetAccountLine")]
    pub budget_account_line: Vec<BudgetAccountLine>,
    #[serde(default, rename = "ReplacedNoticeDocumentReference")]
    pub replaced_notice_document_reference: Option<DocumentReference>,
    #[serde(default, rename = "LotDistribution")]
    pub lot_distribution: Option<LotDistribution>,
    #[serde(default, rename = "PostAwardProcess")]
    pub post_award_process: Option<PostAwardProcess>,
    #[serde(default, rename = "EconomicOperatorShortList")]
    pub economic_operator_short_list: Option<EconomicOperatorShortList>,
    #[serde(default, rename = "SecurityClearanceTerm")]
    pub security_clearance_term: Vec<SecurityClearanceTerm>,
}
