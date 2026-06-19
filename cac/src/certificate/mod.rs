use serde::{Deserialize, Serialize};

pub type CertificateAttachment = crate::Attachment;
pub type CertificateDocumentReference = crate::DocumentReference;

include!("of_origin_application.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct Certificate {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "CertificateTypeCode")]
    pub certificate_type_code: Option<cct::Code>,
    #[serde(default, rename = "CertificateType")]
    pub certificate_type: Vec<cct::Text>,
    #[serde(default, rename = "CertificateReferenceID")]
    pub certificate_reference_id: Option<cct::Identifier>,
    #[serde(default, rename = "ApplicableCategoryCode")]
    pub applicable_category_code: Option<cct::Code>,
    #[serde(default, rename = "ApplicableCategory")]
    pub applicable_category: Option<cct::Text>,
    #[serde(default, rename = "CertificateURI")]
    pub certificate_uri: Option<cct::Identifier>,
    #[serde(default, rename = "Remarks")]
    pub remarks: Vec<cct::Text>,
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: Option<crate::Party>,
    #[serde(default, rename = "CertificateValidityPeriod")]
    pub certificate_validity_period: Option<crate::Period>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<crate::Signature>,
}
