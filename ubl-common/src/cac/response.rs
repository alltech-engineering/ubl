// UBL Response and Status aggregates.

use serde::{Deserialize, Serialize};
use crate::cbc::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub reference_id: Option<ID>,
    pub response_code: Option<ResponseCode>,
    pub description: Vec<Description>,
    pub effective_date: Option<EffectiveDate>,
    pub effective_time: Option<Time>,
    pub status: Vec<Status>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub condition_code: Option<ConditionCode>,
    pub reference_date: Option<ReferenceDate>,
    pub reference_time: Option<ReferenceTime>,
    pub description: Vec<Description>,
    pub status_reason_code: Option<Code>,
    pub status_reason: Vec<Text>,
    pub sequence_id: Option<SequenceID>,
    pub text: Vec<Text>,
    pub indication_indicator: Option<Indicator>,
    pub percent: Option<Percent>,
}
