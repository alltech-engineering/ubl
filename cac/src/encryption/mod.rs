use serde::{Deserialize, Serialize};


include!("symmetric_algorithm.rs");
include!("data.rs");

#[derive(Debug, Deserialize, Serialize)]
/// Details of a certificate path chain used in encryption.
///
/// UBL Dictionary Entry Name: `Encryption Certificate Path Chain. Details`
///
/// Generated from XSD type `EncryptionCertificatePathChainType`.
pub struct EncryptionCertificatePathChain {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The path chain value manifest in the instance.
    #[serde(default, rename = "Value")]
    pub value: Option<cct::Text>,
/// The path chain value references external to the instance.
    #[serde(default, rename = "URI")]
    pub uri: Option<cct::Identifier>,
}
