#[derive(Debug, Deserialize, Serialize)]
pub struct ResultOfVerification {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ValidatorID")]
    pub validator_id: Option<cct::Identifier>,
    #[serde(default, rename = "ValidationResultCode")]
    pub validation_result_code: Option<cct::Code>,
    #[serde(default, rename = "ValidationDate")]
    pub validation_date: Option<udt::DateTime>,
    #[serde(default, rename = "ValidationTime")]
    pub validation_time: Option<udt::DateTime>,
    #[serde(default, rename = "ValidateProcess")]
    pub validate_process: Option<cct::Text>,
    #[serde(default, rename = "ValidateTool")]
    pub validate_tool: Option<cct::Text>,
    #[serde(default, rename = "ValidateToolVersion")]
    pub validate_tool_version: Option<cct::Text>,
    #[serde(default, rename = "SignatoryParty")]
    pub signatory_party: Option<Box<Party>>,
}
