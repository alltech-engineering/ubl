#[derive(Debug, Deserialize, Serialize)]
pub struct KeyInfo {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(rename = "$value")]
    pub content: Vec<KeyInfoTypeContent>,
}
