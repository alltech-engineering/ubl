// UBL DiscrepancyResponse aggregate — response to a discrepancy.
// UBL element: cac:DiscrepancyResponse

use serde::{Deserialize, Serialize};
use crate::cbc::*;
use crate::cac::address::Address;
use crate::cac::party::Party;

/// A class to describe a response to a discrepancy in a business document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscrepancyResponse {
    pub reference_id: Option<ReferenceID>,
    pub response_code: Option<ResponseCode>,
    pub description: Vec<Description>,
    pub effective_date: Option<EffectiveDate>,
    pub effective_time: Option<EffectiveTime>,
    pub note: Vec<Note>,
}
