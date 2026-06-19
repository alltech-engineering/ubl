#[derive(Debug, Deserialize, Serialize)]
pub struct EncryptionSymmetricAlgorithm {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "OID")]
    pub oid: Option<cct::Identifier>,
}
