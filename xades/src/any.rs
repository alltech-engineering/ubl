#[derive(Debug, Deserialize, Serialize)]
pub struct Any {
    #[serde(rename = "@any_attribute")]
    pub any_attribute: String,
    #[serde(default, rename = "$value")]
    pub content: Vec<AnyTypeContent>,
}
