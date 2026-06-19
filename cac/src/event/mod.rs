use serde::{Deserialize, Serialize};

include!("tactic.rs");
include!("line_item.rs");
include!("comment.rs");
include!("tactic_enumeration.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "IdentificationID")]
    pub identification_id: Option<cct::Identifier>,
    #[serde(default, rename = "OccurrenceDate")]
    pub occurrence_date: Option<udt::DateTime>,
    #[serde(default, rename = "OccurrenceTime")]
    pub occurrence_time: Option<udt::DateTime>,
    #[serde(default, rename = "TypeCode")]
    pub type_code: Option<cct::Code>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "CompletionIndicator")]
    pub completion_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "CurrentStatus")]
    pub current_status: Vec<crate::Status>,
    #[serde(default, rename = "Contact")]
    pub contact: Vec<crate::Contact>,
    #[serde(default, rename = "OccurenceLocation")]
    pub occurence_location: Option<crate::Location>,
    #[serde(default, rename = "OccurrenceLocation")]
    pub occurrence_location: Option<crate::Location>,
}
