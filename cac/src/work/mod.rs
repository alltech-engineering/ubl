use serde::{Deserialize, Serialize};


include!("report_line.rs");
include!("phase_reference.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkQuantityTotal {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "Quantity")]
    pub quantity: cct::Quantity,
    #[serde(default, rename = "WorkTypeCode")]
    pub work_type_code: Option<cct::Code>,
    #[serde(default, rename = "WorkTypeDescription")]
    pub work_type_description: Vec<cct::Text>,
}
