#[derive(Debug, Deserialize, Serialize)]
pub struct Object {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(default, rename = "@MimeType")]
    pub mime_type: Option<String>,
    #[serde(default, rename = "@Encoding")]
    pub encoding: Option<String>,
    #[serde(default, rename = "$value")]
    pub content: Vec<ObjectTypeContent>,
}
