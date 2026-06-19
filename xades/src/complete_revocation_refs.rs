#[derive(Debug, Deserialize, Serialize)]
pub struct CompleteRevocationRefs {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(default, rename = "CRLRefs")]
    pub crl_refs: Option<CrlRefsType>,
    #[serde(default, rename = "OCSPRefs")]
    pub ocsp_refs: Option<OcspRefsType>,
    #[serde(default, rename = "OtherRefs")]
    pub other_refs: Option<OtherCertStatusRefsType>,
}
