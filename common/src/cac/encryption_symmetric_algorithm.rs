#[derive(Debug, Deserialize, Serialize)]
pub struct EncryptionSymmetricAlgorithm {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "OID")]
    pub oid: Option<super::cct::IdentifierType>,
}
