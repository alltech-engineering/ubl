#[derive(Debug, Deserialize, Serialize)]
pub struct Signature {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "ReasonCode")]
    pub reason_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "ValidationDate")]
    pub validation_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ValidationTime")]
    pub validation_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ValidatorID")]
    pub validator_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "CanonicalizationMethod")]
    pub canonicalization_method: Option<super::cct::TextType>,
    #[serde(default, rename = "SignatureMethod")]
    pub signature_method: Option<super::cct::TextType>,
    #[serde(default, rename = "SignatoryParty")]
    pub signatory_party: Option<Box<Party>>,
    #[serde(default, rename = "DigitalSignatureAttachment")]
    pub digital_signature_attachment: Option<Attachment>,
    #[serde(default, rename = "OriginalDocumentReference")]
    pub original_document_reference:
        Option<Box<DocumentReference>>,
}
