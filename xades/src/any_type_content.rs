#[derive(Debug, Deserialize, Serialize)]
pub struct AnyTypeContent {
    #[serde(default, rename = "$text")]
    pub text_before: Option<String>,
    #[serde(rename = "any56")]
    pub any: String,
    #[serde(default, rename = "$text")]
    pub text_after_any_56: Option<String>,
}
