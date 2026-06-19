use serde::{Deserialize, Serialize};

pub type CertificateAttachment = crate::Attachment;
pub type CertificateDocumentReference = crate::DocumentReference;

include!("of_origin_application.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to define a certificate applied to the item. Certificated can be a requirement to sell goods
/// or services in a jurisdiction.
///
/// UBL Dictionary Entry Name: `Certificate. Details`
///
/// Generated from XSD type `CertificateType`.
pub struct Certificate {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this certificate.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// The type of this certificate, expressed as a code. The type specifies what array it belongs to,
/// e.g.. Environmental, security, health improvement etc.
    #[serde(default, rename = "CertificateTypeCode")]
    pub certificate_type_code: Option<cct::Code>,
/// The type of this certificate, expressed as a text.
    #[serde(default, rename = "CertificateType")]
    pub certificate_type: Vec<cct::Text>,
/// An identifier assigned by the issuing authority to reference this certificate in an external
/// registry or official record.
    #[serde(default, rename = "CertificateReferenceID")]
    pub certificate_reference_id: Option<cct::Identifier>,
/// A code specifying the category of item or process to which this certificate applies.
    #[serde(default, rename = "ApplicableCategoryCode")]
    pub applicable_category_code: Option<cct::Code>,
/// A textual description of the category of item or process to which this certificate applies.
    #[serde(default, rename = "ApplicableCategory")]
    pub applicable_category: Option<cct::Text>,
/// A textual description of the category of item or process to which this certificate applies.
    #[serde(default, rename = "CertificateURI")]
    pub certificate_uri: Option<cct::Identifier>,
/// Remarks by the applicant for this certificate.
    #[serde(default, rename = "Remarks")]
    pub remarks: Vec<cct::Text>,
/// The authorised Organisation who issues this Certificate.
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: Option<crate::Party>,
/// The period during which this certificate is valid.
    #[serde(default, rename = "CertificateValidityPeriod")]
    pub certificate_validity_period: Option<crate::Period>,
/// A reference to a document relevant to this certificate or an application for this certificate.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<crate::DocumentReference>,
/// A signature applied to this certificate.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<crate::Signature>,
}
