#[derive(Debug, Deserialize, Serialize)]
/// Details of a symmetric algorithm used in encryption.
///
/// UBL Dictionary Entry Name: `Encryption Symmetric Algorithm. Details`
///
/// Generated from XSD type `EncryptionSymmetricAlgorithmType`.
pub struct EncryptionSymmetricAlgorithm {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A human-readable identifier the algorithm.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The object identifier for the algorithm.
    #[serde(default, rename = "OID")]
    pub oid: Option<cct::Identifier>,
}
