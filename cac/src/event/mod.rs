use serde::{Deserialize, Serialize};

include!("tactic.rs");
include!("line_item.rs");
include!("comment.rs");
include!("tactic_enumeration.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a significant occurrence relating to an object, process, or person.
///
/// UBL Dictionary Entry Name: `Event. Details`
///
/// Generated from XSD type `EventType`.
pub struct Event {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this event within an agreed event identification scheme.
    #[serde(default, rename = "IdentificationID")]
    pub identification_id: Option<cct::Identifier>,
/// The date of this event.
    #[serde(default, rename = "OccurrenceDate")]
    pub occurrence_date: Option<udt::DateTime>,
/// The time of this event.
    #[serde(default, rename = "OccurrenceTime")]
    pub occurrence_time: Option<udt::DateTime>,
/// A code signifying the type of this event.
    #[serde(default, rename = "TypeCode")]
    pub type_code: Option<cct::Code>,
/// Text describing this event.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// An indicator that this event has been completed (true) or not (false).
    #[serde(default, rename = "CompletionIndicator")]
    pub completion_indicator: Option<udt::Indicator>,
/// The current status of this event.
    #[serde(default, rename = "CurrentStatus")]
    pub current_status: Vec<crate::Status>,
/// Contacts associated with this event.
    #[serde(default, rename = "Contact")]
    pub contact: Vec<crate::Contact>,
/// (Deprecated) The location of this event.
    #[serde(default, rename = "OccurenceLocation")]
    pub occurence_location: Option<crate::Location>,
/// The location of this event.
    #[serde(default, rename = "OccurrenceLocation")]
    pub occurrence_location: Option<crate::Location>,
}
