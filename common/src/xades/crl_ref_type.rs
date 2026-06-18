#[derive(Debug, Deserialize, Serialize)]
pub struct CrlRefType {
    #[serde(rename = "DigestAlgAndValue")]
    pub digest_alg_and_value: DigestAlgAndValueType,
    #[serde(default, rename = "CRLIdentifier")]
    pub crl_identifier: Option<CrlIdentifierType>,
}
