use serde::{Deserialize, Serialize};


include!("agreement_terms.rs");
include!("collaboration.rs");
include!("service.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a digital trade process.
///
/// UBL Dictionary Entry Name: `Digital Process. Details`
///
/// Generated from XSD type `DigitalProcessType`.
pub struct DigitalProcess {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for the digital collaboration.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// Text describing the digital process.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// Identifies a user-defined profile of this digital process (e.g. an UBL profile).
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::Identifier>,
/// The digital collaboration associated with this digital process.
    #[serde(default, rename = "DigitalCollaboration")]
    pub digital_collaboration: Vec<DigitalCollaboration>,
/// A reference to a certification document associated with this digital process.
    #[serde(default, rename = "CertificationDocumentReference")]
    pub certification_document_reference: Vec<crate::DocumentReference>,
}
