#[derive(Debug, Deserialize, Serialize)]
pub struct CrlRefsType {
    #[serde(default, rename = "CRLRef")]
    pub crl_ref: Vec<CrlRefType>,
}
