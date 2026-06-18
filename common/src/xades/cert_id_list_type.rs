#[derive(Debug, Deserialize, Serialize)]
pub struct CertIdListType {
    #[serde(default, rename = "Cert")]
    pub cert: Vec<CertIdType>,
}
