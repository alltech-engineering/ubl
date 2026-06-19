#[derive(Debug, Deserialize, Serialize)]
pub struct X509Digest {
    #[serde(rename = "@Algorithm")]
    pub algorithm: String,
    #[serde(rename = "$text")]
    pub content: String,
}
