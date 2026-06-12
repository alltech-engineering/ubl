// Status — UBL CAC aggregate
// The condition or position of an object.
use crate::cbc::*;

#[derive(Debug, Clone, Partialserde::Serialize, serde::Deserialize)]
pub struct Status {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_code: Option<ConditionCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_date: Option<ReferenceDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_time: Option<ReferenceTime>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub description: Vec<Description>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason_code: Option<StatusReasonCode>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub status_reason: Vec<StatusReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_id: Option<SequenceID>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub text: Vec<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indication_indicator: Option<IndicationIndicator>,
}
/// Text type for Status
pub type Text = crate::cbc::Description;
