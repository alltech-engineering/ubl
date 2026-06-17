// UBL Dimension aggregate.

use crate::cbc::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dimension {
    pub attribute_id: AttributeID,
    #[serde(default)]
    pub measure: Option<Measure>,
    #[serde(default)]
    pub description: Vec<Description>,
    #[serde(default)]
    pub minimum_measure: Option<Measure>,
    #[serde(default)]
    pub maximum_measure: Option<Measure>,
}
