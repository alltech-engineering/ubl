#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the result of an attempt to verify a signature.
///
/// UBL Dictionary Entry Name: `Result Of Verification. Details`
///
/// Generated from XSD type `ResultOfVerificationType`.
pub struct ResultOfVerification {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for the organization, person, service, or server that verified the signature.
    #[serde(default, rename = "ValidatorID")]
    pub validator_id: Option<cct::Identifier>,
/// A code signifying the result of the verification.
    #[serde(default, rename = "ValidationResultCode")]
    pub validation_result_code: Option<cct::Code>,
/// The date upon which verification took place.
    #[serde(default, rename = "ValidationDate")]
    pub validation_date: Option<udt::DateTime>,
/// The time at which verification took place.
    #[serde(default, rename = "ValidationTime")]
    pub validation_time: Option<udt::DateTime>,
/// The verification process.
    #[serde(default, rename = "ValidateProcess")]
    pub validate_process: Option<cct::Text>,
/// The tool used to verify the signature.
    #[serde(default, rename = "ValidateTool")]
    pub validate_tool: Option<cct::Text>,
/// The version of the tool used to verify the signature.
    #[serde(default, rename = "ValidateToolVersion")]
    pub validate_tool_version: Option<cct::Text>,
/// The signing party.
    #[serde(default, rename = "SignatoryParty")]
    pub signatory_party: Option<Box<Party>>,
}
