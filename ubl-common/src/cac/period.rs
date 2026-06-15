// UBL Period aggregate — a date range.

use crate::cbc::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Period {
    #[serde(default)]
    pub start_date: Option<StartDate>,
    #[serde(default)]
    pub start_time: Option<StartTime>,
    #[serde(default)]
    pub end_date: Option<EndDate>,
    #[serde(default)]
    pub end_time: Option<Time>,
    #[serde(default)]
    pub duration_measure: Option<DurationMeasure>,
    #[serde(default)]
    pub description_code: Vec<Code>,
    #[serde(default)]
    pub description: Vec<Description>,
}
