#[derive(Debug, Deserialize, Serialize)]
pub struct ContractingSystem {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "ContractingSystemTypeCode")]
    pub contracting_system_type_code: Option<cct::Code>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}
