use serde::{Deserialize, Serialize};


include!("value.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe an application-level response to a document.
///
/// UBL Dictionary Entry Name: `Response. Details`
///
/// Generated from XSD type `ResponseType`.
pub struct Response {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for the section (or line) of the document to which this response applies.
    #[serde(default, rename = "ReferenceID")]
    pub reference_id: Option<cct::Identifier>,
/// A code signifying the type of response.
    #[serde(default, rename = "ResponseCode")]
    pub response_code: Option<cct::Code>,
/// Text describing this response.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// The date upon which this response is valid.
    #[serde(default, rename = "EffectiveDate")]
    pub effective_date: Option<udt::DateTime>,
/// The time at which this response is valid.
    #[serde(default, rename = "EffectiveTime")]
    pub effective_time: Option<udt::DateTime>,
/// A status report associated with this response.
    #[serde(default, rename = "Status")]
    pub status: Vec<crate::Status>,
}
