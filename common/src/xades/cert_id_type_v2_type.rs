#[derive(Debug, Deserialize, Serialize)]
pub struct CertIdTypeV2Type {
    #[serde(default, rename = "@URI")]
    pub uri: Option<String>,
    #[serde(rename = "CertDigest")]
    pub cert_digest: DigestAlgAndValueType,
    #[serde(default, rename = "IssuerSerialV2")]
    pub issuer_serial_v2: Option<String>,
}
