#[derive(Debug, Deserialize, Serialize)]
pub struct Text {
    #[serde(default, rename = "@languageID")]
    pub language_id: Option<String>,
    #[serde(default, rename = "@languageLocaleID")]
    pub language_locale_id: Option<String>,
    #[serde(default, rename = "$text")]
    pub content: String,
}
