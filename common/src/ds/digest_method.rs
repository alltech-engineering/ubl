#[derive(Debug, Deserialize, Serialize)]
pub struct DigestMethod {
    #[serde(rename = "@Algorithm")]
    pub algorithm: String,
    #[serde(default, rename = "$text")]
    pub text_before: Option<String>,
    #[serde(default, rename = "any13")]
    pub any: Vec<String>,
}
