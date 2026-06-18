#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessJustification {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "PreviousCancellationReasonCode")]
    pub previous_cancellation_reason_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ProcessReasonCode")]
    pub process_reason_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ProcessReason")]
    pub process_reason: Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
}
