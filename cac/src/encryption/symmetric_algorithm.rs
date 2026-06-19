#[derive(Debug, Deserialize, Serialize)]
pub struct EncryptionSymmetricAlgorithm {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "OID")]
    pub oid: Option<cct::Identifier>,
}
