// UBL Period aggregate — a date range.

use serde::{Deserialize, Serialize};
use crate::cbc::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Period {
    pub start_date: Option<StartDate>,
    pub start_time: Option<StartTime>,
    pub end_date: Option<EndDate>,
    pub end_time: Option<Time>,
    pub duration_measure: Option<DurationMeasure>,
    pub description_code: Vec<Code>,
    pub description: Vec<Description>,
}
