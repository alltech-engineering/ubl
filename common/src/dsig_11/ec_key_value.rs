#[derive(Debug, Deserialize, Serialize)]
pub struct EcKeyValue {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(rename = "$value")]
    pub content: Vec<EcKeyValueTypeContent>,
}
