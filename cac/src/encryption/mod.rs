use serde::{Deserialize, Serialize};


include!("symmetric_algorithm.rs");
include!("data.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct EncryptionCertificatePathChain {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Value")]
    pub value: Option<cct::Text>,
    #[serde(default, rename = "URI")]
    pub uri: Option<cct::Identifier>,
}
