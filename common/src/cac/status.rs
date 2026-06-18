#[derive(Debug, Deserialize, Serialize)]
pub struct Status {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ConditionCode")]
    pub condition_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ReferenceDate")]
    pub reference_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ReferenceTime")]
    pub reference_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "StatusReasonCode")]
    pub status_reason_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "StatusReason")]
    pub status_reason: Vec<super::cct::TextType>,
    #[serde(default, rename = "SequenceID")]
    pub sequence_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Text")]
    pub text: Vec<super::cct::TextType>,
    #[serde(default, rename = "IndicationIndicator")]
    pub indication_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "Percent")]
    pub percent: Option<super::cct::NumericType>,
    #[serde(default, rename = "ReliabilityPercent")]
    pub reliability_percent: Option<super::cct::NumericType>,
    #[serde(default, rename = "DocumentationAttachment")]
    pub documentation_attachment: Vec<Attachment>,
    #[serde(default, rename = "SubStatus")]
    pub sub_status: Vec<Status>,
    #[serde(default, rename = "Condition")]
    pub condition: Vec<Condition>,
}
