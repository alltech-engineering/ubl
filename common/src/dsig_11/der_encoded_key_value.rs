#[derive(Debug, Deserialize, Serialize)]
pub struct DerEncodedKeyValue {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(rename = "$text")]
    pub content: String,
}
