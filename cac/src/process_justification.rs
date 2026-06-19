#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessJustification {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "PreviousCancellationReasonCode")]
    pub previous_cancellation_reason_code: Option<cct::Code>,
    #[serde(default, rename = "ProcessReasonCode")]
    pub process_reason_code: Option<cct::Code>,
    #[serde(default, rename = "ProcessReason")]
    pub process_reason: Vec<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}
