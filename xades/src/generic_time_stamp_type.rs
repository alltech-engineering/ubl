#[derive(Debug, Deserialize, Serialize)]
pub struct GenericTimeStampType {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(rename = "$value")]
    pub content: Vec<GenericTimeStampTypeContent>,
}
