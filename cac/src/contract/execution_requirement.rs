#[derive(Debug, Deserialize, Serialize)]
pub struct ContractExecutionRequirement {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Name")]
    pub name: Vec<cct::Text>,
    #[serde(default, rename = "ExecutionRequirementCode")]
    pub execution_requirement_code: Option<cct::Code>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}
