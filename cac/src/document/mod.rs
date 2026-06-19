use serde::{Deserialize, Serialize};

pub type DocumentLineReference = crate::LineReference;

include!("distribution.rs");
include!("metadata.rs");
include!("reference.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe an application-level response to a document.
///
/// UBL Dictionary Entry Name: `Document Response. Details`
///
/// Generated from XSD type `DocumentResponseType`.
pub struct DocumentResponse {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// A response to the document as a whole.
    #[serde(rename = "Response")]
    pub response: crate::Response,
/// A referenced document.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
/// The Party who issues this Document.
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: Option<crate::Party>,
/// The Party who is the intended recipient of this Document.
    #[serde(default, rename = "RecipientParty")]
    pub recipient_party: Option<crate::Party>,
/// A response to a particular line in the document.
    #[serde(default, rename = "LineResponse")]
    pub line_response: Vec<crate::LineResponse>,
}
