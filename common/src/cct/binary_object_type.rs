#[derive(Debug, Deserialize, Serialize)]
pub struct BinaryObjectType {
    #[serde(default, rename = "@format")]
    pub format: Option<String>,
    #[serde(default, rename = "@mimeCode")]
    pub mime_code: Option<String>,
    #[serde(default, rename = "@encodingCode")]
    pub encoding_code: Option<String>,
    #[serde(default, rename = "@characterSetCode")]
    pub character_set_code: Option<String>,
    #[serde(default, rename = "@uri")]
    pub uri: Option<String>,
    #[serde(default, rename = "@filename")]
    pub filename: Option<String>,
    #[serde(rename = "$text")]
    pub content: String,
}
