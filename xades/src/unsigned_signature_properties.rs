#[derive(Debug, Deserialize, Serialize)]
pub struct UnsignedSignatureProperties {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(rename = "$value")]
    pub content: Vec<UnsignedSignaturePropertiesTypeContent>,
}
