#[derive(Debug, Deserialize, Serialize)]
pub struct DateTime {
    #[serde(default, rename = "@format")]
    pub format: Option<String>,
    #[serde(default, rename = "$text")]
    pub content: String,
}
