use serde::{Deserialize, Serialize};


include!("agreement_terms.rs");
include!("collaboration.rs");
include!("service.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct DigitalProcess {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::Identifier>,
    #[serde(default, rename = "DigitalCollaboration")]
    pub digital_collaboration: Vec<DigitalCollaboration>,
    #[serde(default, rename = "CertificationDocumentReference")]
    pub certification_document_reference: Vec<crate::DocumentReference>,
}
