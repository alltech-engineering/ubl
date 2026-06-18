#[derive(Debug, Deserialize, Serialize)]
pub struct KeyValue {
    #[serde(default, rename = "$text")]
    pub text_before: Option<String>,
    #[serde(rename = "content")]
    pub content: KeyValueTypeContent,
}
