#[derive(Debug, Deserialize, Serialize)]
/// A class to define a signature.
///
/// UBL Dictionary Entry Name: `Signature. Details`
///
/// Generated from XSD type `SignatureType`.
pub struct Signature {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this signature.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// A code defining the reason or purpose of this signature
    #[serde(default, rename = "ReasonCode")]
    pub reason_code: Option<cct::Code>,
/// Free-form text conveying information that is not contained explicitly in other structures; in
/// particular, information regarding the circumstances in which the signature is being used.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// The date upon which this signature was verified.
    #[serde(default, rename = "ValidationDate")]
    pub validation_date: Option<udt::DateTime>,
/// The time at which this signature was verified.
    #[serde(default, rename = "ValidationTime")]
    pub validation_time: Option<udt::DateTime>,
/// An identifier for the organization, person, service, or server that verified this signature.
    #[serde(default, rename = "ValidatorID")]
    pub validator_id: Option<cct::Identifier>,
/// The method used to perform XML canonicalization of this signature.
    #[serde(default, rename = "CanonicalizationMethod")]
    pub canonicalization_method: Option<cct::Text>,
/// Text describing the method of signature.
    #[serde(default, rename = "SignatureMethod")]
    pub signature_method: Option<cct::Text>,
/// The Party that provides the signature.
    #[serde(default, rename = "SignatoryParty")]
    pub signatory_party: Option<Box<Party>>,
/// The actual encoded signature (e.g., in XMLDsig format).
    #[serde(default, rename = "DigitalSignatureAttachment")]
    pub digital_signature_attachment: Option<Attachment>,
/// A reference to the document that the signature applies to. For evidentiary purposes, this may be the
/// document image that the signatory party saw when applying their signature.
    #[serde(default, rename = "OriginalDocumentReference")]
    pub original_document_reference:
        Option<Box<DocumentReference>>,
}
