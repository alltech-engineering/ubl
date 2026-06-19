#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a requirement for execution of a contract.
///
/// UBL Dictionary Entry Name: `Contract Execution Requirement. Details`
///
/// Generated from XSD type `ContractExecutionRequirementType`.
pub struct ContractExecutionRequirement {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// A name for this requirement.
    #[serde(default, rename = "Name")]
    pub name: Vec<cct::Text>,
/// A code signifying a type of requirement to be fulfiled by the economic operator.
    #[serde(default, rename = "ExecutionRequirementCode")]
    pub execution_requirement_code: Option<cct::Code>,
/// Text describing this requirement.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}
