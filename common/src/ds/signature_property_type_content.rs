#[derive(Debug, Deserialize, Serialize)]
pub enum SignaturePropertyTypeContent {
    #[serde(rename = "any35")]
    Any(String),
    #[serde(rename = "$text")]
    Text(String),
}
