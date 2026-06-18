#[derive(Debug, Deserialize, Serialize)]
pub struct ResultOfVerification {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ValidatorID")]
    pub validator_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ValidationResultCode")]
    pub validation_result_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ValidationDate")]
    pub validation_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ValidationTime")]
    pub validation_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ValidateProcess")]
    pub validate_process: Option<super::cct::TextType>,
    #[serde(default, rename = "ValidateTool")]
    pub validate_tool: Option<super::cct::TextType>,
    #[serde(default, rename = "ValidateToolVersion")]
    pub validate_tool_version: Option<super::cct::TextType>,
    #[serde(default, rename = "SignatoryParty")]
    pub signatory_party: Option<Box<Party>>,
}
