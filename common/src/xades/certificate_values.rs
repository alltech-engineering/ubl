#[derive(Debug, Deserialize, Serialize)]
pub struct CertificateValues {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(default, rename = "$value")]
    pub content: Vec<CertificateValuesTypeContent>,
}
