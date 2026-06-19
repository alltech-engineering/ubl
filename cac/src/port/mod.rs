use serde::{Deserialize, Serialize};

pub type PortLocation = crate::Location;

include!("call.rs");
include!("call_record.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the purpose of a port call.
///
/// UBL Dictionary Entry Name: `Port Call Purpose. Details`
///
/// Generated from XSD type `PortCallPurposeType`.
pub struct PortCallPurpose {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The purpose of this port call, expressed as a code.
    #[serde(default, rename = "PurposeTypeCode")]
    pub purpose_type_code: Option<cct::Code>,
/// The purpose of this port call, expressed as a text.
    #[serde(default, rename = "PurposeType")]
    pub purpose_type: Vec<cct::Text>,
/// A description of the purpose of the port call.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}
