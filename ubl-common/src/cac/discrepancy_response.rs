// UBL DiscrepancyResponse aggregate — response to a discrepancy.
// UBL element: cac:DiscrepancyResponse

use crate::cbc::*;
use serde::{Deserialize, Serialize};

/// A class to describe a response to a discrepancy in a business document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscrepancyResponse {
    #[serde(default)]
    pub reference_id: Option<ReferenceID>,
    #[serde(default)]
    pub response_code: Option<ResponseCode>,
    #[serde(default)]
    pub description: Vec<Description>,
    #[serde(default)]
    pub effective_date: Option<EffectiveDate>,
    #[serde(default)]
    pub effective_time: Option<EffectiveTime>,
    #[serde(default)]
    pub note: Vec<Note>,
}
