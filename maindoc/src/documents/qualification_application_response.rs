#[derive(Debug, Deserialize, Serialize)]
pub struct QualificationApplicationResponse {
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
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(rename = "ContractFolderID")]
    pub contract_folder_id: cct::Identifier,
    #[serde(default, rename = "ContractName")]
    pub contract_name: Vec<cct::Text>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
    #[serde(default, rename = "EconomicOperatorGroupName")]
    pub economic_operator_group_name: Option<cct::Text>,
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: Option<cct::Identifier>,
    #[serde(default, rename = "ProcedureCode")]
    pub procedure_code: Option<cct::Code>,
    #[serde(default, rename = "QualificationApplicationTypeCode")]
    pub qualification_application_type_code: Option<cct::Code>,
    #[serde(default, rename = "WeightScoringMethodologyNote")]
    pub weight_scoring_methodology_note: Vec<cct::Text>,
    #[serde(default, rename = "WeightingTypeCode")]
    pub weighting_type_code: Option<cct::Code>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: Vec<cac::ContractingParty>,
    #[serde(default, rename = "EconomicOperatorParty")]
    pub economic_operator_party: Vec<cac::EconomicOperatorParty>,
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: Option<cac::ProcurementProject>,
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: Vec<cac::ProcurementProjectLot>,
    #[serde(default, rename = "TenderingCriterion")]
    pub tendering_criterion: Vec<cac::TenderingCriterion>,
    #[serde(default, rename = "TenderingCriterionResponse")]
    pub tendering_criterion_response: Vec<cac::TenderingCriterionResponse>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Evidence")]
    pub evidence: Vec<cac::Evidence>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
