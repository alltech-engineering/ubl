use serde::{Deserialize, Serialize};


include!("report_line.rs");
include!("phase_reference.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to define a document-level total of reported work expressed as a quantity.
///
/// UBL Dictionary Entry Name: `Work Quantity Total. Details`
///
/// Generated from XSD type `WorkQuantityTotalType`.
pub struct WorkQuantityTotal {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The total quantity for this entry.
    #[serde(rename = "Quantity")]
    pub quantity: cct::Quantity,
/// A code specifying the kind of work quantity being totaled.
    #[serde(default, rename = "WorkTypeCode")]
    pub work_type_code: Option<cct::Code>,
/// A description of what is totaled.
    #[serde(default, rename = "WorkTypeDescription")]
    pub work_type_description: Vec<cct::Text>,
}
