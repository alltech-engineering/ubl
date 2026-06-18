#[derive(Debug, Deserialize, Serialize)]
pub struct KeyInfoReference {
    #[serde(rename = "@URI")]
    pub uri: String,
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
}
