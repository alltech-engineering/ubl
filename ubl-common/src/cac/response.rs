// UBL Response and Status aggregates.

use serde::{Deserialize, Serialize};
use crate::cbc::*;
use crate::cac::document::DocumentReference;
use crate::cac::line::LineReference;
use crate::cac::party::Party;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub reference_id: Option<ID>,
    pub response_code: Option<ResponseCode>,
    #[serde(default)]
    pub description: Vec<Description>,
    pub effective_date: Option<EffectiveDate>,
    pub effective_time: Option<Time>,
    #[serde(default)]
    pub status: Vec<Status>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub condition_code: Option<ConditionCode>,
    pub reference_date: Option<ReferenceDate>,
    pub reference_time: Option<ReferenceTime>,
    #[serde(default)]
    pub description: Vec<Description>,
    pub status_reason_code: Option<Code>,
    #[serde(default)]
    pub status_reason: Vec<Text>,
    pub sequence_id: Option<SequenceID>,
    #[serde(default)]
    pub text: Vec<Text>,
    pub indication_indicator: Option<Indicator>,
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
