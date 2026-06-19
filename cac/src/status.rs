#[derive(Debug, Deserialize, Serialize)]
pub struct Status {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ConditionCode")]
    pub condition_code: Option<cct::Code>,
    #[serde(default, rename = "ReferenceDate")]
    pub reference_date: Option<udt::DateTime>,
    #[serde(default, rename = "ReferenceTime")]
    pub reference_time: Option<udt::DateTime>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "StatusReasonCode")]
    pub status_reason_code: Option<cct::Code>,
    #[serde(default, rename = "StatusReason")]
    pub status_reason: Vec<cct::Text>,
    #[serde(default, rename = "SequenceID")]
    pub sequence_id: Option<cct::Identifier>,
    #[serde(default, rename = "Text")]
    pub text: Vec<cct::Text>,
    #[serde(default, rename = "IndicationIndicator")]
    pub indication_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "Percent")]
    pub percent: Option<cct::Numeric>,
    #[serde(default, rename = "ReliabilityPercent")]
    pub reliability_percent: Option<cct::Numeric>,
    #[serde(default, rename = "DocumentationAttachment")]
    pub documentation_attachment: Vec<Attachment>,
    #[serde(default, rename = "SubStatus")]
    pub sub_status: Vec<Status>,
    #[serde(default, rename = "Condition")]
    pub condition: Vec<Condition>,
}
