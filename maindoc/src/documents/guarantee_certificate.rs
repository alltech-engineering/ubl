#[derive(Debug, Deserialize, Serialize)]
pub struct GuaranteeCertificate {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::IdentifierType>,
    #[serde(rename = "ContractFolderID")]
    pub contract_folder_id: cct::IdentifierType,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTimeType>,
    #[serde(default, rename = "GuaranteeTypeCode")]
    pub guarantee_type_code: Option<cct::CodeType>,
    #[serde(default, rename = "Purpose")]
    pub purpose: Vec<cct::TextType>,
    #[serde(rename = "LiabilityAmount")]
    pub liability_amount: cct::AmountType,
    #[serde(default, rename = "ConstitutionCode")]
    pub constitution_code: Option<cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::TextType>,
    #[serde(default, rename = "ApplicablePeriod")]
    pub applicable_period: Option<cac::Period>,
    #[serde(default, rename = "ApplicableRegulation")]
    pub applicable_regulation: Vec<cac::Regulation>,
    #[serde(default, rename = "GuaranteeDocumentReference")]
    pub guarantee_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "ImmobilizedSecurity")]
    pub immobilized_security: Vec<cac::ImmobilizedSecurity>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(rename = "GuarantorParty")]
    pub guarantor_party: cac::Party,
    #[serde(rename = "InterestedParty")]
    pub interested_party: cac::Party,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Option<cac::Party>,
}
