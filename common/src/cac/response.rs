// UBL Response and Status aggregates.

use crate::cac::document::DocumentReference;
use crate::cac::line::LineReference;
use crate::cac::party::Party;
use crate::cbc::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub reference_id: Option<ID>,
    #[serde(default)]
    pub response_code: Option<ResponseCode>,
    #[serde(default)]
    pub description: Vec<Description>,
    #[serde(default)]
    pub effective_date: Option<EffectiveDate>,
    #[serde(default)]
    pub effective_time: Option<Time>,
    #[serde(default)]
    pub status: Vec<Status>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    #[serde(default)]
    pub condition_code: Option<ConditionCode>,
    #[serde(default)]
    pub reference_date: Option<ReferenceDate>,
    #[serde(default)]
    pub reference_time: Option<ReferenceTime>,
    #[serde(default)]
    pub description: Vec<Description>,
    #[serde(default)]
    pub status_reason_code: Option<Code>,
    #[serde(default)]
    pub status_reason: Vec<Text>,
    #[serde(default)]
    pub sequence_id: Option<SequenceID>,
    #[serde(default)]
    pub text: Vec<Text>,
    #[serde(default)]
    pub indication_indicator: Option<Indicator>,
    #[serde(default)]
    pub percent: Option<Percent>,
}

/// UBL DocumentResponse — response to a specific document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentResponse {
    pub response: Response,
    #[serde(default)]
    pub document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub issuer_party: Vec<Party>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recipient_party: Vec<Party>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub line_response: Vec<LineResponse>,
}

/// UBL LineResponse — response to a specific line.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineResponse {
    pub line_reference: LineReference,
    #[serde(default)]
    pub response: Vec<Response>,
}
