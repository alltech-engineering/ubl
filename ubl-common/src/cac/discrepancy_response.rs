// UBL DiscrepancyResponse aggregate — response to a discrepancy.
// UBL element: cac:DiscrepancyResponse

use serde::{Deserialize, Serialize};
use crate::cbc::*;

/// A class to describe a response to a discrepancy in a business document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscrepancyResponse {
    pub reference_id: Option<ReferenceID>,
    pub response_code: Option<ResponseCode>,
    #[serde(default)]
    pub description: Vec<Description>,
    pub effective_date: Option<EffectiveDate>,
    pub effective_time: Option<EffectiveTime>,
    #[serde(default)]
    pub note: Vec<Note>,
}
