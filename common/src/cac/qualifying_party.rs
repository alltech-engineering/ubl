#[derive(Debug, Deserialize, Serialize)]
pub struct QualifyingParty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ParticipationPercent")]
    pub participation_percent: Option<super::cct::NumericType>,
    #[serde(default, rename = "PersonalSituation")]
    pub personal_situation: Vec<super::cct::TextType>,
    #[serde(default, rename = "OperatingYearsQuantity")]
    pub operating_years_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "EmployeeQuantity")]
    pub employee_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "BusinessClassificationEvidenceID")]
    pub business_classification_evidence_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "BusinessIdentityEvidenceID")]
    pub business_identity_evidence_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "TendererRoleCode")]
    pub tenderer_role_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "BusinessClassificationScheme")]
    pub business_classification_scheme: Option<ClassificationScheme>,
    #[serde(default, rename = "TechnicalCapability")]
    pub technical_capability: Vec<Capability>,
    #[serde(default, rename = "FinancialCapability")]
    pub financial_capability: Vec<Capability>,
    #[serde(default, rename = "CompletedTask")]
    pub completed_task: Vec<CompletedTask>,
    #[serde(default, rename = "Declaration")]
    pub declaration: Vec<Declaration>,
    #[serde(default, rename = "Party")]
    pub party: Option<Party>,
    #[serde(default, rename = "EconomicOperatorRole")]
    pub economic_operator_role: Option<EconomicOperatorRole>,
}
