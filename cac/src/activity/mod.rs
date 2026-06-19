use serde::{Deserialize, Serialize};

pub type ActivityFinalLocation = crate::Location;
pub type ActivityOriginLocation = crate::Location;
pub type ActivityPeriod = crate::Period;

include!("data_line.rs");
include!("property.rs");
