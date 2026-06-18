#[derive(Debug, Deserialize, Serialize)]
pub struct IndicatorType {
    #[serde(default, rename = "@format")]
    pub format: Option<String>,
    #[serde(default, rename = "$text")]
    pub content: String,
}
