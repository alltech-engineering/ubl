#[derive(Debug, Deserialize, Serialize)]
/// Details of an encryption process
///
/// UBL Dictionary Entry Name: `Encryption Data. Details`
///
/// Generated from XSD type `EncryptionDataType`.
pub struct EncryptionData {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The format of the encrypted message.
    #[serde(rename = "MessageFormat")]
    pub message_format: cct::Text,
/// A reference to the certificate used in the encryption process.
    #[serde(default, rename = "EncryptionCertificateAttachment")]
    pub encryption_certificate_attachment: Option<crate::Attachment>,
/// A reference to the path chain defined for the encryption process.
    #[serde(default, rename = "EncryptionCertificatePathChain")]
    pub encryption_certificate_path_chain: Vec<EncryptionCertificatePathChain>,
/// A reference to the symmetric algorithm used for the encryption process.
    #[serde(default, rename = "EncryptionSymmetricAlgorithm")]
    pub encryption_symmetric_algorithm: Vec<EncryptionSymmetricAlgorithm>,
}
