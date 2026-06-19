#[derive(Debug, Deserialize, Serialize)]
pub struct OcspRefsType {
    #[serde(default, rename = "OCSPRef")]
    pub ocsp_ref: Vec<OcspRefType>,
}
