#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the condition or position of an object.
///
/// UBL Dictionary Entry Name: `Status. Details`
///
/// Generated from XSD type `StatusType`.
pub struct Status {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// Specifies the status condition of the related object.
    #[serde(default, rename = "ConditionCode")]
    pub condition_code: Option<cct::Code>,
/// The reference date for this status.
    #[serde(default, rename = "ReferenceDate")]
    pub reference_date: Option<udt::DateTime>,
/// The reference time for this status.
    #[serde(default, rename = "ReferenceTime")]
    pub reference_time: Option<udt::DateTime>,
/// Text describing this status.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// The reason for this status condition or position, expressed as a code.
    #[serde(default, rename = "StatusReasonCode")]
    pub status_reason_code: Option<cct::Code>,
/// The reason for this status condition or position, expressed as text.
    #[serde(default, rename = "StatusReason")]
    pub status_reason: Vec<cct::Text>,
/// A sequence identifier for this status.
    #[serde(default, rename = "SequenceID")]
    pub sequence_id: Option<cct::Identifier>,
/// Provides any textual information related to this status.
    #[serde(default, rename = "Text")]
    pub text: Vec<cct::Text>,
/// Specifies an indicator relevant to a specific status.
    #[serde(default, rename = "IndicationIndicator")]
    pub indication_indicator: Option<udt::Indicator>,
/// A percentage meaningful in the context of this status.
    #[serde(default, rename = "Percent")]
    pub percent: Option<cct::Numeric>,
/// The reliability of this status, expressed as a percentage.
    #[serde(default, rename = "ReliabilityPercent")]
    pub reliability_percent: Option<cct::Numeric>,
/// One or more attachments (such as photos) used to document the status of the object.
    #[serde(default, rename = "DocumentationAttachment")]
    pub documentation_attachment: Vec<Attachment>,
/// An additional sub status to clarify or ellaborate on the status
    #[serde(default, rename = "SubStatus")]
    pub sub_status: Vec<Status>,
/// Measurements that quantify the condition of the objects covered by the status.
    #[serde(default, rename = "Condition")]
    pub condition: Vec<Condition>,
}
