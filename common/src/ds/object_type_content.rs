#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectTypeContent {
    #[serde(default, rename = "$text")]
    pub text_before: Option<String>,
    #[serde(rename = "any31")]
    pub any: String,
    #[serde(default, rename = "$text")]
    pub text_after_any_31: Option<String>,
}
