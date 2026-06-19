#[derive(Debug, Deserialize, Serialize)]
pub struct OcspRefType {
    #[serde(rename = "OCSPIdentifier")]
    pub ocsp_identifier: OcspIdentifierType,
    #[serde(default, rename = "DigestAlgAndValue")]
    pub digest_alg_and_value: Option<DigestAlgAndValueType>,
}
