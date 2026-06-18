#[derive(Debug, Deserialize, Serialize)]
pub struct CompleteCertificateRefs {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(rename = "CertRefs")]
    pub cert_refs: CertIdListType,
}
