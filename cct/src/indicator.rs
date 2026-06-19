#[derive(Debug, Deserialize, Serialize)]
pub struct Indicator {
    #[serde(default, rename = "@format")]
    pub format: Option<String>,
    #[serde(default, rename = "$text")]
    pub content: String,
}
