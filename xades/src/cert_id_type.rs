#[derive(Debug, Deserialize, Serialize)]
pub struct CertIdType {
    #[serde(default, rename = "@URI")]
    pub uri: Option<String>,
    #[serde(rename = "CertDigest")]
    pub cert_digest: DigestAlgAndValueType,
    #[serde(rename = "IssuerSerial")]
    pub issuer_serial: ds::X509IssuerSerial,
}
