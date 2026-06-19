use serde::{Deserialize, Serialize};

pub type DocumentLineReference = crate::LineReference;

include!("distribution.rs");
include!("metadata.rs");
include!("reference.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentResponse {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(rename = "Response")]
    pub response: crate::Response,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: Option<crate::Party>,
    #[serde(default, rename = "RecipientParty")]
    pub recipient_party: Option<crate::Party>,
    #[serde(default, rename = "LineResponse")]
    pub line_response: Vec<crate::LineResponse>,
}
