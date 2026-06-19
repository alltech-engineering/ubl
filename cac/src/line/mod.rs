use serde::{Deserialize, Serialize};

pub type LinePeriod = crate::Period;

include!("item.rs");
include!("reference.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct LineResponse {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "LineReference")]
    pub line_reference: LineReference,
    #[serde(default, rename = "Response")]
    pub response: Vec<crate::Response>,
}
