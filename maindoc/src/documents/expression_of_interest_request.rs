#[derive(Debug, Deserialize, Serialize)]
pub struct ExpressionOfInterestRequest {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::Identifier>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::Identifier>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::Identifier>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::Identifier>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(default, rename = "ContractFolderID")]
    pub contract_folder_id: Option<cct::Identifier>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
    #[serde(default, rename = "ContractName")]
    pub contract_name: Vec<cct::Text>,
    #[serde(default, rename = "PreferredLanguageLocaleCode")]
    pub preferred_language_locale_code: Option<cct::Code>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<cac::Period>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(rename = "EconomicOperatorParty")]
    pub economic_operator_party: cac::EconomicOperatorParty,
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: Vec<cac::ContractingParty>,
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: Option<cac::ProcurementProject>,
    #[serde(default, rename = "ProcurementProjectLotReference")]
    pub procurement_project_lot_reference: Vec<cac::ProcurementProjectLotReference>,
}
