#[derive(Debug, Deserialize, Serialize)]
pub struct NumericType {
    #[serde(default, rename = "@format")]
    pub format: Option<String>,
    #[serde(rename = "$text")]
    pub content: f64,
}
