use serde::{Deserialize, Serialize};

pub type LinePeriod = crate::Period;

include!("item.rs");
include!("reference.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe responses to a line in a document.
///
/// UBL Dictionary Entry Name: `Line Response. Details`
///
/// Generated from XSD type `LineResponseType`.
pub struct LineResponse {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// A reference to the line being responded to.
    #[serde(rename = "LineReference")]
    pub line_reference: LineReference,
/// A response to the referenced line.
    #[serde(default, rename = "Response")]
    pub response: Vec<crate::Response>,
}
