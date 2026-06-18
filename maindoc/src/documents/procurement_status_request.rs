#[derive(Debug, Deserialize, Serialize)]
pub struct ProcurementStatusRequest {
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
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::IdentifierType>,
    #[serde(default, rename = "ContractFolderID")]
    pub contract_folder_id: Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTimeType>,
    #[serde(default, rename = "ContractName")]
    pub contract_name: Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::TextType>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: Vec<cac::ContractingParty>,
    #[serde(rename = "EconomicOperatorParty")]
    pub economic_operator_party: cac::EconomicOperatorParty,
    #[serde(default, rename = "TenderingProcess")]
    pub tendering_process: Option<cac::TenderingProcess>,
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: Option<cac::ProcurementProject>,
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: Vec<cac::ProcurementProjectLot>,
}
