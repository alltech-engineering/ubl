// Response — UBL CAC aggregate
// An application-level response to a document.
use crate::cbc::*;

#[derive(Debug, Clone, Partialserde::Serialize, serde::Deserialize)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<ReferenceID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<ResponseCode>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub description: Vec<Description>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<EffectiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<EffectiveTime>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub status: Vec<Status>,
}
use super::status::Status;
