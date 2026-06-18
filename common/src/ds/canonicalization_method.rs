#[derive(Debug, Deserialize, Serialize)]
pub struct CanonicalizationMethod {
    #[serde(rename = "@Algorithm")]
    pub algorithm: String,
    #[serde(default, rename = "$text")]
    pub text_before: Option<String>,
    #[serde(default, rename = "any5")]
    pub any: Vec<String>,
}
