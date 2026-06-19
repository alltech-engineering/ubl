use serde::{Deserialize, Serialize};

pub type PortLocation = crate::Location;

include!("call.rs");
include!("call_record.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct PortCallPurpose {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "PurposeTypeCode")]
    pub purpose_type_code: Option<cct::Code>,
    #[serde(default, rename = "PurposeType")]
    pub purpose_type: Vec<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}
