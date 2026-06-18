#[derive(Debug, Deserialize, Serialize)]
pub struct EncapsulatedPkiData {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(default, rename = "@Encoding")]
    pub encoding: Option<String>,
    #[serde(rename = "$text")]
    pub content: String,
}
