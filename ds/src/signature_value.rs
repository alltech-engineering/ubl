#[derive(Debug, Deserialize, Serialize)]
pub struct SignatureValue {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(rename = "$text")]
    pub content: String,
}
