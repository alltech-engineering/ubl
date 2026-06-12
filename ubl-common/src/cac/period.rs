// Period — UBL CAC aggregate
// A date range with start/end.
use crate::cbc::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Period {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<StartDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<StartTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<EndDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<EndTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_measure: Option<Measure>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub description: Vec<Description>,
}
