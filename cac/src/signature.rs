#[derive(Debug, Deserialize, Serialize)]
pub struct Signature {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "ReasonCode")]
    pub reason_code: Option<cct::Code>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "ValidationDate")]
    pub validation_date: Option<udt::DateTime>,
    #[serde(default, rename = "ValidationTime")]
    pub validation_time: Option<udt::DateTime>,
    #[serde(default, rename = "ValidatorID")]
    pub validator_id: Option<cct::Identifier>,
    #[serde(default, rename = "CanonicalizationMethod")]
    pub canonicalization_method: Option<cct::Text>,
    #[serde(default, rename = "SignatureMethod")]
    pub signature_method: Option<cct::Text>,
    #[serde(default, rename = "SignatoryParty")]
    pub signatory_party: Option<Box<Party>>,
    #[serde(default, rename = "DigitalSignatureAttachment")]
    pub digital_signature_attachment: Option<Attachment>,
    #[serde(default, rename = "OriginalDocumentReference")]
    pub original_document_reference:
        Option<Box<DocumentReference>>,
}
