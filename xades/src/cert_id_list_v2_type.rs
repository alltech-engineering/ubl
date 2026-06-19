#[derive(Debug, Deserialize, Serialize)]
pub struct CertIdListV2Type {
    #[serde(default, rename = "Cert")]
    pub cert: Vec<CertIdTypeV2Type>,
}
