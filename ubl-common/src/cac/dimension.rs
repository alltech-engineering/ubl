// UBL Dimension aggregate.

use serde::{Deserialize, Serialize};
use crate::cbc::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dimension {
    pub attribute_id: AttributeID,
    pub measure: Option<Measure>,
    pub description: Vec<Description>,
    pub minimum_measure: Option<Measure>,
    pub maximum_measure: Option<Measure>,
}
