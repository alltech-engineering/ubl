#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectReference {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
    #[serde(default, rename = "WorkPhaseReference")]
    pub work_phase_reference: Vec<WorkPhaseReference>,
}
