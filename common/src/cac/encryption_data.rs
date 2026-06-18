#[derive(Debug, Deserialize, Serialize)]
pub struct EncryptionData {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "MessageFormat")]
    pub message_format: super::cct::TextType,
    #[serde(default, rename = "EncryptionCertificateAttachment")]
    pub encryption_certificate_attachment: Option<Attachment>,
    #[serde(default, rename = "EncryptionCertificatePathChain")]
    pub encryption_certificate_path_chain: Vec<EncryptionCertificatePathChain>,
    #[serde(default, rename = "EncryptionSymmetricAlgorithm")]
    pub encryption_symmetric_algorithm: Vec<EncryptionSymmetricAlgorithm>,
}
