#[derive(Debug, Deserialize, Serialize)]
pub struct ContractExecutionRequirement {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Name")]
    pub name: Vec<super::cct::TextType>,
    #[serde(default, rename = "ExecutionRequirementCode")]
    pub execution_requirement_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
}
