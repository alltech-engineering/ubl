#[derive(Debug, Deserialize, Serialize)]
pub enum TransformTypeContent {
    #[serde(rename = "any11")]
    Any(String),
    #[serde(rename = "XPath")]
    XPath(String),
    #[serde(rename = "$text")]
    Text(String),
}
