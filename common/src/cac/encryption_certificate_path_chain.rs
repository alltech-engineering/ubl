#[derive(Debug, Deserialize, Serialize)]
pub struct EncryptionCertificatePathChain {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Value")]
    pub value: Option<super::cct::TextType>,
    #[serde(default, rename = "URI")]
    pub uri: Option<super::cct::IdentifierType>,
}
