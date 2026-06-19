#[derive(Debug, Deserialize, Serialize)]
pub struct EncryptionData {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "MessageFormat")]
    pub message_format: cct::Text,
    #[serde(default, rename = "EncryptionCertificateAttachment")]
    pub encryption_certificate_attachment: Option<crate::Attachment>,
    #[serde(default, rename = "EncryptionCertificatePathChain")]
    pub encryption_certificate_path_chain: Vec<EncryptionCertificatePathChain>,
    #[serde(default, rename = "EncryptionSymmetricAlgorithm")]
    pub encryption_symmetric_algorithm: Vec<EncryptionSymmetricAlgorithm>,
}
